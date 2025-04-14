import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Color, NewColor } from "../models/colors";

export const useColorTool = () => {
    const [colors, setColors] = useState([] as Color[]);
    const [error, setError] = useState<string | null>(null);

    const refreshColors = useCallback(async () => {
        try {
            const colors = (await invoke("get_colors")) as Color[];
            setColors(colors);
            setError(null);
        } catch (err) {
            console.error(err);
            setError("Failed to fetch colors.");
        }
    }, []);

    const addColor = useCallback(async (newColor: NewColor) => {
        try {
            await invoke("add_color", { newColor });
            await refreshColors();
        } catch (err) {
            console.error(err);
            setError("Failed to add color.");
        }
    }, [refreshColors]);

    const deleteColor = useCallback(async (colorIdToDelete: number) => {
        try {
            await invoke("delete_color", { colorIdToDelete });
            await refreshColors();
        } catch (err) {
            console.error(err);
            setError("Failed to delete color.");
        }
    }, [refreshColors]);

    useEffect(() => {
        refreshColors();
    }, [refreshColors]);

    return [colors, addColor, deleteColor, error] as const;
};