import "@fontsource-variable/inter";
import styles from "./main.module.scss";
import "@radix-ui/themes/styles.css";
import "./global.scss";

import React from "react";
import ReactDOM from "react-dom/client";
import { Flex } from "@radix-ui/themes";

import { Route, Router, Switch } from "wouter";
import { AuthRoutes } from "./pages/auth";
import { KeygateProvider } from "@keygate/react";

const root = document.getElementById("root");
if (!root) throw new Error("Root element not found");

import { DashboardRoutes } from "./pages/dashboard";

const base = "";

ReactDOM.createRoot(root).render(
	<React.StrictMode>
		<KeygateProvider>
			<Flex direction={"column"} className={styles.base}>
				<Router base={base}>
					<Switch>
						<Route path="/auth/:path*" component={AuthRoutes} />
						<Route path="/:path*" component={DashboardRoutes} />
					</Switch>
				</Router>
			</Flex>
		</KeygateProvider>
	</React.StrictMode>,
);
