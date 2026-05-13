#!/usr/bin/env python3
# Font recoloring script for Linuxstrap
# Converts TTF fonts to single-color COLR OTF

import argparse
import os
import shutil
import sys
from pathlib import Path

try:
    from fontTools.ttLib import TTFont
    from fontTools.ttLib.tables.C_O_L_R_ import LayerRecord, table_C_O_L_R_
    from fontTools.ttLib.tables.C_P_A_L_ import Color, table_C_P_A_L_
except ImportError:
    print("fonttools not installed. Run: pip install fonttools")
    sys.exit(1)

SUPPORTED_EXTENSIONS = (".ttf", ".otf")

def hex_to_rgb(hex_str):
    hex_str = hex_str.strip().lstrip("#")
    if len(hex_str) != 6:
        raise ValueError("Hex color must be 6 characters long")
    return (
        int(hex_str[0:2], 16),
        int(hex_str[2:4], 16),
        int(hex_str[4:6], 16),
    )

def get_sober_font_dir():
    return (
        Path.home()
        / ".var"
        / "app"
        / "org.vinegarhq.Sober"
        / "data"
        / "sober"
        / "asset_overlay"
        / "ExtraContent"
        / "LuaPackages"
        / "Packages"
        / "_Index"
        / "BuilderIcons"
        / "BuilderIcons"
        / "Font"
    )

def recolor_font(file_path, rgb_color):
    input_path = Path(file_path)
    output_path = input_path.with_suffix(".otf") if input_path.suffix.lower() != ".otf" else input_path

    try:
        font = TTFont(input_path)

        if "COLR" in font:
            del font["COLR"]

        r, g, b = rgb_color
        cpal = table_C_P_A_L_()
        cpal.version = 0
        cpal.palettes = [[Color(b, g, r, 255)]]
        cpal.numPaletteEntries = 1
        font["CPAL"] = cpal

        colr = table_C_O_L_R_()
        colr.version = 0
        colr.ColorLayers = {}
        for glyph in font.getGlyphOrder():
            if glyph != ".notdef":
                layer = LayerRecord()
                layer.name = glyph
                layer.colorID = 0
                colr.ColorLayers[glyph] = [layer]
        font["COLR"] = colr

        font.save(output_path)
        print(f"Processed: {output_path}")
        return output_path

    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return None

def copy_font_to_sober(output_font_path):
    dest_dir = get_sober_font_dir()
    dest_dir.mkdir(parents=True, exist_ok=True)
    dest_file = dest_dir / Path(output_font_path).name
    shutil.copy2(output_font_path, dest_file)
    print(f"Copied to: {dest_file}")
    return True

def write_buildericons_json():
    font_dir = get_sober_font_dir()
    root = font_dir.parent
    content = """{
        "name": "Builder Icons",
        "loadStrategy": "sameFamilyOnly",
        "faces": [
            {
                "name": "Regular",
                "weight": 400,
                "style": "normal",
                "assetId": "rbxasset://LuaPackages/Packages/_Index/BuilderIcons/BuilderIcons/Font/BuilderIcons-Regular.otf"
            },
            {
                "name": "Bold",
                "weight": 700,
                "style": "normal",
                "assetId": "rbxasset://LuaPackages/Packages/_Index/BuilderIcons/BuilderIcons/Font/BuilderIcons-Bold.otf"
            }
        ]
    }"""
    (root / "BuilderIcons.json").write_text(content, encoding="utf-8")
    print(f"Written BuilderIcons.json to: {root}")
    return True

def process_directory(target_dir, rgb_color, copy_to_sober=True):
    if not os.path.isdir(target_dir):
        print(f"Invalid directory: {target_dir}")
        sys.exit(1)

    count = 0
    for root, _, files in os.walk(target_dir):
        for file in files:
            if file.lower().endswith(SUPPORTED_EXTENSIONS):
                result = recolor_font(os.path.join(root, file), rgb_color)
                if result and copy_to_sober:
                    copy_font_to_sober(result)
                count += 1

    print(f"Processed {count} files")

    if copy_to_sober:
        write_buildericons_json()

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--path", required=True)
    parser.add_argument("--color", required=True)
    parser.add_argument("--mod-name", default="CustomMod")
    args = parser.parse_args()

    try:
        color = hex_to_rgb(args.color)
    except ValueError as e:
        print(e)
        sys.exit(1)

    process_directory(args.path, color)