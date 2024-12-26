import { CssBaseline, ThemeProvider } from "@mui/material";

import { ResponsiveBreakpointProvider } from "./hooks/useResponsiveBreakpoint";
import Interface from "./interface";
import theme from "./theme";

const App = (): JSX.Element => (
    <ThemeProvider theme={theme}>
        <CssBaseline />
        <ResponsiveBreakpointProvider>
            <Interface />
        </ResponsiveBreakpointProvider>
    </ThemeProvider>
);

export default App;
