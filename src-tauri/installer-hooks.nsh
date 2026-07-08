; installer-hooks.nsh — custom NSIS hooks for Open 3D Viewer
;
; Adds a top-level "Open with Open 3D Viewer" entry to the right-click
; context menu for every supported extension, alongside Windows' built-in
; "Open with" submenu. Uses HKCU\Software\Classes\SystemFileAssociations
; so it shows up regardless of which app is the current default handler
; and works with per-user (currentUser) installs — no admin required.
;
; Removed cleanly on uninstall.

!include "LogicLib.nsh"

; Macro: register the "Open with Open 3D Viewer" verb for one extension.
; Arg: the extension (with leading dot), e.g. ".glb"
!macro AddOpen3DViewerContext EXT
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\${EXT}\shell\Open3DViewer" "MUIVerb" "Open with Open 3D Viewer"
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\${EXT}\shell\Open3DViewer" "Icon" '"$INSTDIR\Open 3D Viewer.exe",0'
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\${EXT}\shell\Open3DViewer\command" "" '"$INSTDIR\Open 3D Viewer.exe" "%1"'
!macroend

!macro RemoveOpen3DViewerContext EXT
  DeleteRegKey HKCU "Software\Classes\SystemFileAssociations\${EXT}\shell\Open3DViewer"
!macroend

; ---- POSTINSTALL: add context menu entries ------------------------------
!macro NSIS_HOOK_POSTINSTALL
  DetailPrint "Registering top-level right-click 'Open with Open 3D Viewer' entries..."
  !insertmacro AddOpen3DViewerContext ".glb"
  !insertmacro AddOpen3DViewerContext ".gltf"
  !insertmacro AddOpen3DViewerContext ".fbx"
  !insertmacro AddOpen3DViewerContext ".obj"
  !insertmacro AddOpen3DViewerContext ".stl"
  !insertmacro AddOpen3DViewerContext ".ply"
  !insertmacro AddOpen3DViewerContext ".usdz"
  !insertmacro AddOpen3DViewerContext ".usd"
  !insertmacro AddOpen3DViewerContext ".usda"
  !insertmacro AddOpen3DViewerContext ".usdc"
  !insertmacro AddOpen3DViewerContext ".step"
  !insertmacro AddOpen3DViewerContext ".stp"
  !insertmacro AddOpen3DViewerContext ".iges"
  !insertmacro AddOpen3DViewerContext ".igs"
  !insertmacro AddOpen3DViewerContext ".brep"
  !insertmacro AddOpen3DViewerContext ".scad"
  !insertmacro AddOpen3DViewerContext ".hdr"
  !insertmacro AddOpen3DViewerContext ".exr"
!macroend

; ---- PREUNINSTALL: remove context menu entries --------------------------
!macro NSIS_HOOK_PREUNINSTALL
  DetailPrint "Removing 'Open with Open 3D Viewer' context menu entries..."
  !insertmacro RemoveOpen3DViewerContext ".glb"
  !insertmacro RemoveOpen3DViewerContext ".gltf"
  !insertmacro RemoveOpen3DViewerContext ".fbx"
  !insertmacro RemoveOpen3DViewerContext ".obj"
  !insertmacro RemoveOpen3DViewerContext ".stl"
  !insertmacro RemoveOpen3DViewerContext ".ply"
  !insertmacro RemoveOpen3DViewerContext ".usdz"
  !insertmacro RemoveOpen3DViewerContext ".usd"
  !insertmacro RemoveOpen3DViewerContext ".usda"
  !insertmacro RemoveOpen3DViewerContext ".usdc"
  !insertmacro RemoveOpen3DViewerContext ".step"
  !insertmacro RemoveOpen3DViewerContext ".stp"
  !insertmacro RemoveOpen3DViewerContext ".iges"
  !insertmacro RemoveOpen3DViewerContext ".igs"
  !insertmacro RemoveOpen3DViewerContext ".brep"
  !insertmacro RemoveOpen3DViewerContext ".scad"
  !insertmacro RemoveOpen3DViewerContext ".hdr"
  !insertmacro RemoveOpen3DViewerContext ".exr"
!macroend
