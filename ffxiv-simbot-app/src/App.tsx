import { BrowserRouter as Router, Route, Routes, Link } from "react-router-dom";
import { Logo } from "./components/image/Logo";
import "./App.css";
import { SimulationResult } from "./page/SimulationResult";
import { QuickSim } from "./page/QuickSim";
import { Home } from "./page/home";
import { Typography, Box } from "@mui/material";


export const QuickSimInputSaveName = "mostRecentQuickSimInput";
export const QuickSimResponseSaveName = "mostRecentQuickSimResponse";
export const StatCompareRequestSaveName = "mostRecentStatCompareRequest";
export const StatCompareResponseSaveName = "mostRecentStatCompareResponse";


function App() {
  return (
        <Router>
      <div className="App">
        <header className="App-header">
          <Link to="/">
            <Logo />
          </Link>
        </header>

        <main className="Body">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/quicksim" element={<QuickSim />} />
            <Route path="/simulationresult" element={<SimulationResult />} />
          </Routes>
        </main>
        <Box className="Footer" component="footer">
          <Typography variant="caption">
            <p>Creator: Fly Ninetynine@Aegis(Elemental DC)</p>
            <p>Email: ns090200@gmail.com</p>
            <p>License: MIT</p>
          </Typography>
        </Box>
      </div>
    </Router>
  );
}

export default App;
