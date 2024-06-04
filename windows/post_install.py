#!/usr/bin/env python3

from typing import Any
import os
import shutil
import functools


def print_wrapper(message="Base"):
    def decorator(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            print(
                "------------------------------------------------------------------------------"
            )
            print(message)
            func(*args, **kwargs)
            print("Succesfully done")
            print(
                "------------------------------------------------------------------------------"
            )

        return wrapper

    return decorator


def copytree(src, dst, symlinks=False, ignore=None):
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            shutil.copytree(s, d, symlinks, ignore, dirs_exist_ok=True)
        else:
            shutil.copy2(s, d)


BASE_PATH = "\\".join([i for i in os.popen("where meson")][0].split("\\")[:-2])
SOURCE_ROOT = os.environ["MESON_SOURCE_ROOT"].replace("/", "\\")
INSTALL_PREFIX = os.environ["MESON_INSTALL_PREFIX"]
BIN_DIR = os.path.join(INSTALL_PREFIX, "bin")
SHARE_DIR = os.path.join(INSTALL_PREFIX, "share")
ICONS_DIR = os.path.join(SHARE_DIR, "icons")
LIB_DIR = os.path.join(INSTALL_PREFIX, "lib")


class DepsFinder:
    deps_list: list[str] = []

    def find_msys_deps(self, target: str, depth: int = 0) -> None:
        if depth == 0:
            return
        for line in os.popen(f'ntldd "{target}"'):
            line = " ".join(line.split()[2:3])
            if line == "not":
                continue
            path: list[str] = line.split("\\")
            lib: str = path[-1]
            if path[1] == "Windows":
                continue
            if lib not in self.deps_list:
                self.deps_list.append(lib)
                self.find_msys_deps(lib, depth - 1)


@print_wrapper(message="Copying dependencies")
def copy_dlls(dll_list: list[str]) -> None:
    for dep in dll_list:
        if os.path.exists(BIN_DIR):
            if os.path.exists(os.path.join(BASE_PATH, "bin", dep)):
                shutil.copy(
                    os.path.join(BASE_PATH, "bin", dep), os.path.join(BIN_DIR, dep)
                )


@print_wrapper(message="Copying icons")
def copy_icons(icon_names: list[str]) -> None:
    if not os.path.exists(SHARE_DIR):
        os.mkdir(SHARE_DIR)
    if not os.path.exists(ICONS_DIR):
        os.mkdir(ICONS_DIR)
    for icon_name in icon_names:
        dest_dir = os.path.join(ICONS_DIR, icon_name)
        src_dir = os.path.join(BASE_PATH, "share", "icons", icon_name)
        if not os.path.exists(src_dir):
            continue
        copytree(src_dir, dest_dir)


@print_wrapper(message="Copy libs deps")
def copy_gdk_pixbuff():
    if not os.path.exists(LIB_DIR):
        os.mkdir(LIB_DIR)
    gdk_pix_path = os.path.join(LIB_DIR, "gdk-pixbuf-2.0")
    gettext_path = os.path.join(LIB_DIR, "gettext")
    if not os.path.exists(gdk_pix_path):
        os.mkdir(gdk_pix_path)
    if not os.path.exists(gettext_path):
        os.mkdir(gettext_path)
    copytree(os.path.join(BASE_PATH, "lib", "gdk-pixbuf-2.0"), gdk_pix_path)
    copytree(os.path.join(BASE_PATH, "lib", "gettext"), gettext_path)


@print_wrapper(message="Searching deps")
def clear_deps_search(func, *args):
    func(*args)


@print_wrapper(message="Search deps for libs")
def search_libs_deps(finder, *args):
    for lib in args:
        lib_path: str = os.path.join(BASE_PATH, "lib", lib)
        dlls = filter(lambda dll: dll.split(".")[-1] == "dll", os.listdir(lib_path))
        for dll in dlls:
            finder(os.path.join(lib_path, dll), 3)


def get_all_content_of_dir(path: str) -> dict[Any, Any] | None | str:
    result = dict()
    if path.split("/")[-1] == "applications":
        result[os.path.join(path, "mimeinfo.cache")] = "file"
    if path.split("/")[-1] == "schemas":
        result[os.path.join(path, "gschemas.compiled")] = "file"
    if not os.path.exists(path):
        return None
    if os.path.isfile(path):
        return None
    for name in os.listdir(path):
        content_path = os.path.join(path, name)
        if os.path.isdir(content_path):
            result[content_path] = get_all_content_of_dir(content_path)
        if os.path.isfile(content_path):
            result[content_path] = "file"
    if len(result) == 0:
        return "empty dir"
    else:
        return result


def install_files(files: dict[Any, Any], path: str) -> str:
    result = ""
    path = path.replace(INSTALL_PREFIX, "")
    if len(path) > 0 and path[0] == "/":
        path = path[1:]
    try:
        [i for i in files.values()].index("file")
        temp = f'SetOutPath "$INSTDIR\\{path}"'.replace("/", "\\")
        if temp[-2] == "\\":
            temp = temp[-2] + '"'
        result += temp + "\n"
        for key, val in files.items():
            if val == "file":
                temp = f"{key}".replace("/", "\\")
                result += f'File "{temp}"\n'
    except ValueError:
        pass
    if len(files) != 0:
        for key, val in files.items():
            if isinstance(val, dict):
                result += install_files(val, key)
    return result


def uninstall_files(content: dict[Any, Any]) -> tuple[str | Any, Any]:
    files = ""
    dirs = ""
    for key, val in content.items():
        key = key.replace(INSTALL_PREFIX, "$INSTDIR").replace("/", "\\")
        if isinstance(val, dict):
            dirs += '"' + key + '"\n'
            temp = uninstall_files(val)
            files += temp[0]
            dirs += temp[1]
        elif val == "file":
            files += 'Delete "' + key + '"\n'
    return (files, dirs)


@print_wrapper(message="Generating installer building script")
def create_nsis_script():
    content = get_all_content_of_dir(INSTALL_PREFIX)
    with open(os.path.join(INSTALL_PREFIX, "gciphers-rs.nsi"), "wt") as file:
        file.write(f'''
!define APP_NAME "GCiphers-rs"
!define COMP_NAME "Sidecuter"
!define WEB_SITE "https://github.com/sidecuter/gciphers-rs"
!define VERSION "1.0.0.0"
!define COPYRIGHT "Sidecuter © 2024"
!define DESCRIPTION "Application"
!define LICENSE_TXT "{SOURCE_ROOT}\\COPYING"
!define INSTALLER_NAME "{SOURCE_ROOT}\\gciphers-rs.exe"
!define MAIN_APP_EXE "bin\\gciphers-rs.exe"
!define INSTALL_TYPE "SetShellVarContext all"
!define REG_ROOT "HKLM"''')
        file.write("""
!define REG_APP_PATH "Software\\Microsoft\\Windows\\CurrentVersion\\App Paths\\${MAIN_APP_EXE}"
!define UNINSTALL_PATH "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\${APP_NAME}"
!define REG_START_MENU "Start Menu Folder"

var SM_Folder

######################################################################

VIProductVersion  "${VERSION}"
VIAddVersionKey "ProductName"  "${APP_NAME}"
VIAddVersionKey "CompanyName"  "${COMP_NAME}"
VIAddVersionKey "LegalCopyright"  "${COPYRIGHT}"
VIAddVersionKey "FileDescription"  "${DESCRIPTION}"
VIAddVersionKey "FileVersion"  "${VERSION}"

######################################################################

SetCompressor ZLIB
Name "${APP_NAME}"
Caption "${APP_NAME}"
OutFile "${INSTALLER_NAME}"
BrandingText "${APP_NAME}"
InstallDirRegKey "${REG_ROOT}" "${REG_APP_PATH}" ""
InstallDir "C:\\Program Files (x86)\\GCiphers-rs"

######################################################################

!include "MUI.nsh"

!define MUI_ABORTWARNING
!define MUI_UNABORTWARNING

!define MUI_LANGDLL_REGISTRY_ROOT "${REG_ROOT}"
!define MUI_LANGDLL_REGISTRY_KEY "${UNINSTALL_PATH}"
!define MUI_LANGDLL_REGISTRY_VALUENAME "Installer Language"

!insertmacro MUI_PAGE_WELCOME

!ifdef LICENSE_TXT
!insertmacro MUI_PAGE_LICENSE "${LICENSE_TXT}"
!endif

!ifdef REG_START_MENU
!define MUI_STARTMENUPAGE_DEFAULTFOLDER "GCiphers-rs"
!define MUI_STARTMENUPAGE_REGISTRY_ROOT "${REG_ROOT}"
!define MUI_STARTMENUPAGE_REGISTRY_KEY "${UNINSTALL_PATH}"
!define MUI_STARTMENUPAGE_REGISTRY_VALUENAME "${REG_START_MENU}"
!insertmacro MUI_PAGE_STARTMENU Application $SM_Folder
!endif

!insertmacro MUI_PAGE_INSTFILES

!define MUI_FINISHPAGE_RUN "$INSTDIR\\${MAIN_APP_EXE}"
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM

!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "English"
!insertmacro MUI_LANGUAGE "Russian"

!insertmacro MUI_RESERVEFILE_LANGDLL

######################################################################

Function .onInit
!insertmacro MUI_LANGDLL_DISPLAY
FunctionEnd

######################################################################

Section
; include for some of the windows messages defines
!include "winmessages.nsh"
; HKLM (all users) vs HKCU (current user) defines
!define env_hklm 'HKLM "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment"'
!define env_hkcu 'HKCU "Environment"'
; set variable for local machine
WriteRegExpandStr ${env_hklm} GCAD $INSTDIR
; and current user
WriteRegExpandStr ${env_hkcu} GCAD $INSTDIR
; make sure windows knows about the change
SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

######################################################################

Section -MainProgram
${INSTALL_TYPE}
SetOverwrite ifnewer
""")
        file.write(install_files(content, INSTALL_PREFIX))
        file.write("""
SectionEnd

######################################################################

Section -Icons_Reg
SetOutPath "$INSTDIR"
WriteUninstaller "$INSTDIR\\uninstall.exe"

!ifdef REG_START_MENU
!insertmacro MUI_STARTMENU_WRITE_BEGIN Application
CreateDirectory "$SMPROGRAMS\\$SM_Folder"
CreateShortCut "$SMPROGRAMS\\$SM_Folder\\${APP_NAME}.lnk" "$INSTDIR\\${MAIN_APP_EXE}"
CreateShortCut "$DESKTOP\\${APP_NAME}.lnk" "$INSTDIR\\${MAIN_APP_EXE}"
CreateShortCut "$SMPROGRAMS\\$SM_Folder\\Uninstall ${APP_NAME}.lnk" "$INSTDIR\\uninstall.exe"

!insertmacro MUI_STARTMENU_WRITE_END
!endif

!ifndef REG_START_MENU
CreateDirectory "$SMPROGRAMS\\GCiphers-rs"
CreateShortCut "$SMPROGRAMS\\GCiphers-rs\\${APP_NAME}.lnk" "$INSTDIR\\${MAIN_APP_EXE}"
CreateShortCut "$DESKTOP\\${APP_NAME}.lnk" "$INSTDIR\\${MAIN_APP_EXE}"
CreateShortCut "$SMPROGRAMS\\GCiphers-rs\\Uninstall ${APP_NAME}.lnk" "$INSTDIR\\uninstall.exe"

!endif

WriteRegStr ${REG_ROOT} "${REG_APP_PATH}" "" "$INSTDIR\\${MAIN_APP_EXE}"
WriteRegStr ${REG_ROOT} "${UNINSTALL_PATH}"  "DisplayName" "${APP_NAME}"
WriteRegStr ${REG_ROOT} "${UNINSTALL_PATH}"  "UninstallString" "$INSTDIR\\uninstall.exe"
WriteRegStr ${REG_ROOT} "${UNINSTALL_PATH}"  "DisplayIcon" "$INSTDIR\\${MAIN_APP_EXE}"
WriteRegStr ${REG_ROOT} "${UNINSTALL_PATH}"  "DisplayVersion" "${VERSION}"
WriteRegStr ${REG_ROOT} "${UNINSTALL_PATH}"  "Publisher" "${COMP_NAME}"

SectionEnd

######################################################################

Section Uninstall
${INSTALL_TYPE}                   
""")
        temp = uninstall_files(content)
        temp1 = temp[1].split()
        temp1.sort(key=lambda x: len(x.split("\\")), reverse=True)
        file.write(temp[0])
        file.write("RmDir " + "\nRmDir ".join(temp1))
        file.write("""
Delete "$INSTDIR\\uninstall.exe"

RmDir "$INSTDIR"

!ifdef REG_START_MENU
!insertmacro MUI_STARTMENU_GETFOLDER "Application" $SM_Folder
Delete "$SMPROGRAMS\\$SM_Folder\\${APP_NAME}.lnk"
Delete "$SMPROGRAMS\\$SM_Folder\\Uninstall ${APP_NAME}.lnk"
Delete "$DESKTOP\\${APP_NAME}.lnk"

RmDir "$SMPROGRAMS\\$SM_Folder"
!endif

!ifndef REG_START_MENU
Delete "$SMPROGRAMS\\GCiphers-rs\\${APP_NAME}.lnk"
Delete "$SMPROGRAMS\\GCiphers-rs\\Uninstall ${APP_NAME}.lnk"
Delete "$DESKTOP\\${APP_NAME}.lnk"

RmDir "$SMPROGRAMS\\GCiphers-rs"
!endif

; delete variable
DeleteRegValue ${env_hklm} GCAD
DeleteRegValue ${env_hkcu} GCAD
; make sure windows knows about the change
SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000

DeleteRegKey ${REG_ROOT} "${REG_APP_PATH}"
DeleteRegKey ${REG_ROOT} "${UNINSTALL_PATH}"
SectionEnd

######################################################################

Function un.onInit
!insertmacro MUI_UNGETLANGUAGE
FunctionEnd

######################################################################
""")


def main():
    deps_finder = DepsFinder()
    clear_deps_search(
        deps_finder.find_msys_deps,
        INSTALL_PREFIX + "/bin/gciphers-rs.exe",
        8,
    )
    deps_finder.deps_list.append("gdbus.exe")
    deps_finder.deps_list.append("gettext.exe")
    search_libs_deps(deps_finder.find_msys_deps, "gdk-pixbuf-2.0\\2.10.0\\loaders")
    copy_dlls(deps_finder.deps_list)
    copy_icons(["Adwaita", "hicolor"])
    copy_gdk_pixbuff()
    create_nsis_script()


if __name__ == "__main__":
    main()
