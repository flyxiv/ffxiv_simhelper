import { BrowserRouter as Router, Route, Routes, Link } from "react-router-dom";
import { ExploreSim } from "./page/ExploreSim";
import { Logo } from "./components/image/Logo";
import { Box, Typography } from "@mui/material";
import "@fontsource/roboto/700.css";
import "./App.css";
import { SimulationResult } from "./page/SimulationResult";
import { StatCompare } from "./page/StatCompare";
import { StatCompareResult } from "./page/StatCompareResult";
import { QuickSim } from "./page/QuickSim";
import { Home } from "./page/home";

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
            <Route path="/exploresim" element={<ExploreSim />} />
            <Route path="/simulationresult" element={<SimulationResult />} />
            <Route path="/statcompare" element={<StatCompare />} />
            <Route path="/statcompareresult" element={<StatCompareResult />} />
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
