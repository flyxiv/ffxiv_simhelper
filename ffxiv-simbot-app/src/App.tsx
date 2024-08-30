import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import "./App.css";
import { SimulationResult } from "./page/SimulationResult";
import { QuickSim } from "./page/QuickSim";
import { GearCompare } from "./page/GearCompare";
import { Home } from "./page/home";
import { StatWeights } from "./page/StatWeights";
import { BestPartner } from "./page/BestPartner";


export const SINGLE_INPUT_SAVE_NAME = "mostRecentSingleInput";
export const BEST_PARTNER_INPUT_SAVE_NAME = "mostRecentBestPartnerInput";
export const GEAR_COMPARE_REQUEST_SAVE_NAME = "mostRecentGearCompareRequest";

export const QUICK_SIM_RESPONSE_SAVE_NAME = "mostRecentQuickSimResponse";
export const GEAR_COMPARE_RESPONSE_SAVE_NAME = "mostRecentGearCompareResponse";
export const BEST_PARTNER_RESPONSE_SAVE_NAME = "mostRecentBestPartnerResponse";
export const STAT_WEIGHTS_RESPONSE_SAVE_NAME = "mostRecentStatWeightsResponse";

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
          <Route path={`/${BEST_PARTNER_URL}`} element={<BestPartner />} />
          <Route path={`/${STAT_WEIGHTS_URL}`} element={<StatWeights />} />
        </Routes>
      </main>
    </Router>
  );
}

export default App;
