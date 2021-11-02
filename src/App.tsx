import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { CssBaseline, ThemeProvider } from "@mui/material";
import theme from "./theme";
import Navbar from "./components/Navbar";
import Signin from "./pages/loggedout/Signin";

function App() {
  return (
    <Router>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <Switch>
          {/* <Route path="/" component={Navbar} /> */}
          <Route path="/signin" component={Signin} />
        </Switch>
      </ThemeProvider>
    </Router>
  );
}

export default App;
