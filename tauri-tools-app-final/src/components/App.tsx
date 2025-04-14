import { BrowserRouter, Routes, Route } from "react-router";
import { Home } from "./Home";
import { ColorTool } from "./ColorTool";
import { Layout } from "./Layout";

export const App = () => {
    return (
        <BrowserRouter>
            <Layout>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/color-tool" element={<ColorTool />} />        
                </Routes>
            </Layout>
        </BrowserRouter>
    );
};