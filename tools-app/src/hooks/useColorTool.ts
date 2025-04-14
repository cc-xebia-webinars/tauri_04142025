import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export const useColorTool = () => {

    const [colors, setColors] = useState([] as string[]);

    const refreshColors = useCallback(async () => {
        const colors = (await invoke("get_colors")) as string[];
        console.log(colors);
        setColors(colors);
    }, []);

    const addColor = useCallback(async (newColor: string) => {
        await invoke("add_color", { newColor: newColor });
        await refreshColors();
    }, [refreshColors]);

    const deleteColor = useCallback(async (colorToDelete: string) => {
        await invoke("delete_color", { colorToDelete });
        await refreshColors();
    }, [refreshColors]);

    useEffect(() => {
        refreshColors();
    }, [refreshColors]);

    return [colors, addColor, deleteColor] as const;
};