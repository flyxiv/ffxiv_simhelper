import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import "./App.css";
import { SimulationResult } from "./page/SimulationResult";
import { QuickSim } from "./page/QuickSim";
import { GearCompare } from "./page/GearCompare";
import { Home } from "./page/home";


export const QuickSimInputSaveName = "mostRecentQuickSimInput";
export const QuickSimResponseSaveName = "mostRecentQuickSimResponse";
export const GearCompareRequestSaveName = "mostRecentGearCompareRequest";
export const GearCompareResponseSaveName = "mostRecentGearCompareResponse";

export const QUICKSIM_URL = "quicksim";
export const QUICKSIM_RESULT_URL = "quicksimsimulationresult";

export const GEAR_COMPARE_URL = "gearcompare";

export const BEST_PARTNER_URL = "bestpartner";

export const STAT_WEIGHTS_URL = "statweights";

export const HOME_PAGE_NAME = "Home";
export const QUICKSIM_PAGE_NAME = "Quick Sim";
export const GEAR_COMPARE_PAGE_NAME = "Gear Compare";
export const BEST_PARTNER_PAGE_NAME = "Best Partner";
export const STAT_WEIGHTS_PAGE_NAME = "Stat Weights";



function App() {
  return (
    <Router>
      <main className="Body">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path={`/${QUICKSIM_URL}`} element={<QuickSim />} />
          <Route path={`/${QUICKSIM_RESULT_URL}`} element={<SimulationResult />} />
          <Route path={`/${GEAR_COMPARE_URL}`} element={<GearCompare />} />
        </Routes>
      </main>
    </Router>
  );
}

export default App;
