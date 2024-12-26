import { CssBaseline, ThemeProvider } from "@mui/material";

import { ResponsiveBreakpointProvider } from "./hooks/useResponsiveBreakpoint";
import { SettingsStoreProvider } from "./hooks/useSettingsStore";
import Interface from "./interface";
import theme from "./theme";

const App = (): JSX.Element => (
    <ThemeProvider theme={theme}>
        <CssBaseline />
        <SettingsStoreProvider>
            <ResponsiveBreakpointProvider>
                <Interface />
            </ResponsiveBreakpointProvider>
        </SettingsStoreProvider>
    </ThemeProvider>
);

export default App;
