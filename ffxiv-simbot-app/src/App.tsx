import { BrowserRouter as Router, Route, Routes, useLocation } from "react-router-dom";
import "./App.css";
import { SimulationResult } from "./page/SimulationResult";
import { QuickSim } from "./page/QuickSim";
import { GearCompare } from "./page/GearCompare";
import { Home } from "./page/home";
import { StatWeights } from "./page/StatWeights";
import { BestPartner } from "./page/BestPartner";
import { GearCompareResult } from "./page/GearCompareResult";
import { BestPartnerResult } from "./page/BestPartnerResult";
import { StatWeightsResult } from "./page/StatWeightsResult";
import { MENU_WIDTH_VW_LG, MENU_WIDTH_VW_XS, MENU_WIDTH_VW_XL, MENU_WIDTH_VW_SM, MENU_WIDTH_VW_MD } from "./components/container/LeftMenu";
import { useEffect } from "react";

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
export const GEAR_COMPARE_RESULT_URL = "gearcomparesimulationresult";

export const BEST_PARTNER_URL = "bestpartner";
export const BEST_PARTNER_RESULT_URL = "bestpartnersimulationresult";

export const STAT_WEIGHTS_URL = "statweights";
export const STAT_WEIGHTS_RESULT_URL = "statweightssimulationresult";



export const BODY_WIDTH = {
  xs: `${100 - MENU_WIDTH_VW_XS}vw`,
  sm: `${100 - MENU_WIDTH_VW_SM}vw`,
  md: `${100 - MENU_WIDTH_VW_MD}vw`,
  lg: `${100 - MENU_WIDTH_VW_LG}vw`,
  xl: `${100 - MENU_WIDTH_VW_XL}vw`,
}


function App() {
  return (
    <Router>
      <ScrollToTop />
      <main className="Body">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path={`/${QUICKSIM_URL}`} element={<QuickSim />} />

          <Route
            path={`/${QUICKSIM_RESULT_URL}`}
            element={<SimulationResult />}
          />

          <Route path={`/${GEAR_COMPARE_URL}`} element={<GearCompare />} />
          <Route
            path={`/${GEAR_COMPARE_RESULT_URL}`}
            element={<GearCompareResult />}
          />

          <Route path={`/${BEST_PARTNER_URL}`} element={<BestPartner />} />
          <Route
            path={`/${BEST_PARTNER_RESULT_URL}`}
            element={<BestPartnerResult />}
          />

          <Route path={`/${STAT_WEIGHTS_URL}`} element={<StatWeights />} />
          <Route
            path={`/${STAT_WEIGHTS_RESULT_URL}`}
            element={<StatWeightsResult />}
          />
        </Routes>
      </main>
    </Router>
  );
}

export default App;

const ScrollToTop = () => {
  const { pathname } = useLocation();

  useEffect(() => {
    window.scrollTo(0, 0);
  }, [pathname]);

  return null;
};