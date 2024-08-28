import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import "./App.css";
import { SimulationResult } from "./page/SimulationResult";
import { QuickSim } from "./page/QuickSim";
import { Home } from "./page/home";


export const QuickSimInputSaveName = "mostRecentQuickSimInput";
export const QuickSimResponseSaveName = "mostRecentQuickSimResponse";
export const StatCompareRequestSaveName = "mostRecentStatCompareRequest";
export const StatCompareResponseSaveName = "mostRecentStatCompareResponse";


function App() {
  return (
    <Router>
      <main className="Body">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/quicksim" element={<QuickSim />} />
          <Route path="/simulationresult" element={<SimulationResult />} />
        </Routes>
      </main>
    </Router>
  );
}

export default App;
