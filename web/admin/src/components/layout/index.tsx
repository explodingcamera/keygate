import { FiGrid, FiSettings, FiUser, FiUsers } from "react-icons/fi";

import { FunctionalComponent } from "preact";
import styled from "@emotion/styled";
import { useLocation } from "wouter-preact";

const SidebarWrapper = styled.div`
  background-color: #0a0a0a;
  overflow-y: auto;
	padding: 1rem;
	display: flex;
	flex-direction: column;
	height: 100%;
	min-width: 15rem;

	h1 {
		font-size: 2em;
		font-weight: 100;
		line-height: normal;
		background: linear-gradient(#8ecef5, #d5eefd, #fff);
			background-clip: border-box;
		-webkit-background-clip: text;
		background-clip: tex;
		-webkit-text-fill-color: transparent;
		padding: 1rem 0;
		padding-top: 0;
	}

	ul {
		list-style: none;
		padding: 0;
		margin: 0;
		color: #c7c7c7;

		li {
			margin-bottom: .5rem;
			padding: 1rem 1rem;
			border-radius: 0.25rem;
			cursor: pointer;
			background-color: #7272721a;
			transition: background-color 0.2s ease-in-out;

			&:hover, &.active {
				background-color: #72727271;

				svg {
					color: #fff;
				}
			}

			svg {
				margin-right: 0.8rem;
				vertical-align: middle;
				color: #727272;
				transition: color 0.2s ease-in-out;
			}
		}
	}

`;

const Credits = styled.div`
	margin-top: auto;
	font-weight: 200;


	h4 {
		font-weight: 100;
		b {
			font-weight: 600;
		}
	}
`;

const Sidebar = () => {
	const [location] = useLocation();
	const isActive = (path: string) =>
		location.startsWith(path) ? "active" : "";

	return (
		<SidebarWrapper>
			<h1>keygate</h1>
			<ul>
				<li className={(location === "/" && "active") || ""}>
					<FiGrid />
					Dashboard
				</li>
				<li className={isActive("/users/")}>
					<FiUsers />
					User Management
				</li>
				<li className={isActive("/portal/")}>
					<FiUsers />
					Auth Portal
				</li>
				<li className={isActive("/monitoring/")}>
					<FiUsers />
					Monitoring
				</li>
				<li className={isActive("/security/")}>
					<FiUsers />
					Security
				</li>
				<li className={isActive("/settings/")}>
					<FiSettings />
					Settings
				</li>
			</ul>

			<Credits>
				<h4>
					<b>keygate.io</b>
					<br />
					Community Edition v0.1.0
				</h4>
			</Credits>
		</SidebarWrapper>
	);
};

const LayoutWrapper = styled.div`
	display: flex;
	> div:last-of-type {
		display: flex;
		flex: 1;
		flex-direction: column;

		main {
			height: 100%;
			overflow: auto;

			> div {
				padding: 1rem;
			}
		}
	}

	height: 100vh;
  height: -webkit-fill-available;
`;

export const Layout: FunctionalComponent<{}> = ({ children }) => {
	return (
		<LayoutWrapper>
			<Sidebar />
			<div>
				<Nav />
				<main>{children}</main>
			</div>
		</LayoutWrapper>
	);
};

const NavWrapper = styled.nav`
	padding: 1rem;
	display: flex;
	flex-direction: row;
	justify-content: space-between;
	align-items: center;
	width: 100%;

	> div {
		display: flex;
		align-items: center;
		svg {
			margin: 0 0.5rem;
		}
	}

	h3 {
		font-weight: 300;
	}

	h1 {
		font-weight: 300;
		font-size: 1.8rem;
	}
`;

const useRouteName = () => {
	const [location] = useLocation();
	const routes: Record<string, string> = {
		"/": "Dashboard",
		"/users/": "User Management",
		"/portal/": "Auth Portal",
		"/monitoring/": "Monitoring",
		"/security/": "Security",
		"/settings/": "Settings",
	};

	return routes[location] || "Dashboard";
};

const Nav = () => {
	const routeName = useRouteName();

	return (
		<NavWrapper>
			<div>
				<h1>{routeName}</h1>
			</div>
			<div>
				<FiUser />
				<h3>Username</h3>
			</div>
		</NavWrapper>
	);
};
