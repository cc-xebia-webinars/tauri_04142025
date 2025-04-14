import { Link } from "react-router";

export const Layout = ({ children }: { children: React.ReactNode }) => {
    return (
        <div className="d-flex flex-column vh-100" style={{ backgroundColor: "black" }}>
            <nav className="bg-dark text-white p-2">
                <ul className="nav">
                    <li className="nav-item">
                        <Link className="nav-link text-white" to="/">Home</Link>
                    </li>
                    <li className="nav-item">
                        <Link className="nav-link text-white" to="/color-tool">ColorTool</Link>
                    </li>
                </ul>
            </nav>
            <main className="flex-grow-1 d-flex align-items-center justify-content-center text-white">
                {children}
            </main>
            <footer className="bg-dark text-white text-center p-2">
                <p className="m-0">Tools App</p>
            </footer>
        </div>
    );
}