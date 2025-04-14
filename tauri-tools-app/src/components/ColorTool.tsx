import { ChangeEvent, useCallback, useState } from "react";
import { useColorTool } from "../hooks/useColorTool";

export const ColorTool = () => {
    const [colors, addColor, removeColor, error] = useColorTool();
    const [colorForm, setColorForm] = useState({
        name: '', hexcode: '',
    });

    const change = useCallback((e: ChangeEvent<HTMLInputElement>) => {
        setColorForm(prevState => ({
            ...prevState,
            [e.target.name]: e.target.value,
        }));
    }, []);

    const doAddColor = useCallback(() => {
        addColor(colorForm);
        setColorForm({
            name: '', hexcode: ''
        });
    }, [colorForm, addColor]);

    return (
        <div id="color-tool" className="container my-5 p-4 bg-dark text-light rounded shadow">
            {error && <div className="alert alert-danger text-center">Error: {error}</div>}
            <header className="text-center mb-4">
                <h1 className="display-5 text-warning">Color Tool</h1>
            </header>
            <ul className="list-group mb-4">
                {!colors.length && (
                    <li className="list-group-item bg-secondary text-light text-center">
                        No Colors
                    </li>
                )}
                {colors.map(color => (
                    <li
                        key={color.id}
                        className="list-group-item bg-secondary text-light d-flex justify-content-between align-items-center"
                    >
                        <span>
                            {color.name} <span>(#{color.hexcode})</span>
                        </span>
                        <button
                            type="button"
                            className="btn btn-danger btn-sm"
                            onClick={() => removeColor(color.id)}
                        >
                            Remove
                        </button>
                    </li>
                ))}
            </ul>
            <form className="row g-3">
                <div className="col-md-6">
                    <label htmlFor="name" className="form-label text-light">Name</label>
                    <input
                        type="text"
                        id="name"
                        name="name"
                        className="form-control bg-dark text-light border-secondary"
                        autoCorrect="off"
                        autoCapitalize="off"
                        value={colorForm.name}
                        onChange={change}
                    />
                </div>
                <div className="col-md-6">
                    <label htmlFor="hexcode" className="form-label text-light">Hexcode</label>
                    <input
                        type="text"
                        id="hexcode"
                        name="hexcode"
                        className="form-control bg-dark text-light border-secondary"
                        autoCorrect="off"
                        autoCapitalize="off"
                        value={colorForm.hexcode}
                        onChange={change}
                    />
                </div>
                <div className="col-12 text-center">
                    <button
                        type="button"
                        className="btn btn-warning"
                        onClick={doAddColor}
                    >
                        Add Color
                    </button>
                </div>
            </form>
        </div>
    );
};