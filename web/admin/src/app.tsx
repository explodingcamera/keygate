import { Layout } from "./components/layout";

import { Route, Router } from "wouter-preact";
import { Home } from "./pages";

export function App() {
	return (
		<Layout>
			<Router>
				<Route path="/" component={Home} />
			</Router>
		</Layout>
	);
}
