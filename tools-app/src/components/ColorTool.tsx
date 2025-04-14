import { ChangeEvent, useCallback, useState } from "react";
import { useColorTool } from "../hooks/useColorTool";
import './ColorTool.css';

export const ColorTool = () => {

  const [colors, addColor, deleteColor] = useColorTool();
  const [newColor, setNewColor] = useState("");

  const change = (event: ChangeEvent<HTMLInputElement>) => {
    setNewColor(event.target.value);
  };

  const doAddColor = useCallback(() => {
    addColor(newColor);
    setNewColor("");
  }, [addColor, newColor]);

  return (
    <div id="color-tool">
      <header>
        <h1>Color Tool</h1>
      </header>
      <ul>
          {!colors.length && <li>No Colors</li>}
          {colors.map(color => <li key={color}>
              {color}
              <button type="button"
                  onClick={() => deleteColor(color)}>X</button>
          </li>)}
      </ul>
      <form>
        <label>
          <span>Name:</span>
          <input type="text" value={newColor} onChange={change} />
        </label>
        <button type="button" onClick={doAddColor}>Add Color</button>
      </form>
    </div>
  );


};