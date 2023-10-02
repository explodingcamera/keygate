import styles from "./main.module.scss";

import React from "react";
import ReactDOM from "react-dom/client";
import { Flex, ThemePanel } from "@radix-ui/themes";

import { Redirect, Route, Router, Switch } from "wouter";
import { AuthRoutes } from "./pages/auth";
import { KeygateProvider } from "@keygate/react";

const root = document.getElementById("root");
if (!root) throw new Error("Root element not found");

import "./global.scss";

const base = "";

ReactDOM.createRoot(root).render(
	<React.StrictMode>
		<KeygateProvider>
			<Flex p={"4"} direction={"column"} className={styles.base}>
				<Router base={base}>
					<Switch>
						<Route path="/">
							<Redirect to="/auth/login" />
						</Route>
						<Route path="/auth/:path*" component={AuthRoutes} />
						<Route component={() => <div>404</div>} />
					</Switch>
				</Router>
			</Flex>
		</KeygateProvider>
	</React.StrictMode>,
);
