import {
  BrowserRouter,
  HashRouter,
  Route,
  Router,
  Routes,
  useLocation,
} from "react-router-dom";
import "./App.css";
import { SimulationResult } from "./page/SimulationResult";
import { DpsAnalysis } from "./page/DpsAnalysis";
import { GearCompare } from "./page/GearCompare";
import { Home } from "./page/home";
import { BestStats } from "./page/BestStats";
import { BestPartner } from "./page/BestPartner";
import { GearCompareResult } from "./page/GearCompareResult";
import { BestPartnerResult } from "./page/BestPartnerResult";
import { StatWeightsResult } from "./page/StatWeightsResult";
import {
  MENU_WIDTH_VW_LG,
  MENU_WIDTH_VW_XS,
  MENU_WIDTH_VW_XL,
  MENU_WIDTH_VW_SM,
  MENU_WIDTH_VW_MD,
} from "./components/container/LeftMenu";
import { useEffect } from "react";
import { createTheme, ThemeProvider } from "@mui/material";
import { AppConfigurations } from "./Themes";

export const SINGLE_INPUT_SAVE_NAME = "mostRecentSingleInput";
export const BEST_PARTNER_INPUT_SAVE_NAME = "mostRecentBestPartnerInput";

export const GEAR_COMPARE_REQUEST_SAVE_NAME = "mostRecentGearCompareRequest";

export const DPS_ANALYSIS_RESPONSE_SAVE_NAME = "mostRecentDpsAnalysisResponse";
export const GEAR_COMPARE_RESPONSE_SAVE_NAME = "mostRecentGearCompareResponse";
export const BEST_PARTNER_RESPONSE_SAVE_NAME = "mostRecentBestPartnerResponse";
export const STAT_WEIGHTS_RESPONSE_SAVE_NAME = "mostRecentBestStatsResponse";

export const DPS_ANALYSIS_URL = "dpsanalysis";
export const DPS_ANALYSIS_RESULT_URL = "dpsanalysissimulationresult";

export const GEAR_COMPARE_URL = "gearcompare";
export const GEAR_COMPARE_RESULT_URL = "gearcomparesimulationresult";

export const BEST_PARTNER_URL = "bestpartner";
export const BEST_PARTNER_RESULT_URL = "bestpartnersimulationresult";

export const BEST_STATS_URL = "beststats";
export const BEST_STATS_RESULT_URL = "beststatssimulationresult";

export const BODY_WIDTH = "100%";
export const HOME_PAGE_MIN_WIDTH_PX = (itemsPerRow: number) => itemsPerRow === 2 ? 1700 : 100;


export const theme = () => createTheme({
  palette: {
    primary: {
      main: "#BB86FC",
    },

    secondary: {
      main: "#03DAC6",
    },
  },

  typography: {
    fontFamily: "NotoSansKR, Arial",
    h1: {
      fontSize: "3rem",
      fontWeight: 900,
    },
    h2: {
      fontSize: "1.75rem",
      fontWeight: 600,
    },
    h3: {
      fontSize: "1.5rem",
      fontWeight: 600,
    },

  },

  components: {
    MuiOutlinedInput: {
      defaultProps: {
        notched: false,
      },
    },
    MuiInputLabel: {
      defaultProps: {
        shrink: false,
      },
    },
  },
});

function App() {
  return (
    <ThemeProvider theme={theme()}>
      {AppConfigurations.isApp ? AppWithHashRouter() : AppWithRouter()}

    </ThemeProvider>

  );
}

export default App;

function AppWithHashRouter() {
  return (
    <HashRouter>
      <ScrollToTop />
      <main className="Body">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path={`/${DPS_ANALYSIS_URL}`} element={<DpsAnalysis />} />

          <Route
            path={`/${DPS_ANALYSIS_RESULT_URL}`}
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

          <Route path={`/${BEST_STATS_URL}`} element={<BestStats />} />
          <Route
            path={`/${BEST_STATS_RESULT_URL}`}
            element={<StatWeightsResult />}
          />
        </Routes>
      </main>
    </HashRouter>
  )
}

function AppWithRouter() {
  return (
    <BrowserRouter>
      <ScrollToTop />
      <main className="Body">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path={`/${DPS_ANALYSIS_URL}`} element={<DpsAnalysis />} />

          <Route
            path={`/${DPS_ANALYSIS_RESULT_URL}`}
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

          <Route path={`/${BEST_STATS_URL}`} element={<BestStats />} />
          <Route
            path={`/${BEST_STATS_RESULT_URL}`}
            element={<StatWeightsResult />}
          />
        </Routes>
      </main>
    </BrowserRouter>
  )
}

const ScrollToTop = () => {
  const { pathname } = useLocation();

  useEffect(() => {
    window.scrollTo(0, 0);
  }, [pathname]);

  return null;
};
