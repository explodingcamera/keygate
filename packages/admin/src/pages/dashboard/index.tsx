import {
	Button,
	Container,
	Flex,
	Box,
	Heading,
	DropdownMenu,
	Avatar,
	Text,
} from "@radix-ui/themes";

import { Link, Route, Router, Switch, useLocation, useRouter } from "wouter";

import { Overview } from "./overview";
import styles from "./dashboard.module.scss";

export const DashboardRoutes = () => {
	const router = useRouter();

	return (
		<Router base="" parent={router}>
			<DashboardLayout>
				<Switch>
					<Route path="/" component={Overview} />
					<Route path="/users">users</Route>
					<Route path="/settings">settings</Route>
					<Route path="/applications">apps</Route>
					<Route component={() => <div>404</div>} />
				</Switch>
			</DashboardLayout>
		</Router>
	);
};

const DashboardLayout = ({ children }: { children: React.ReactNode }) => {
	const router = useRouter();
	const [location] = useLocation();
	const currentPath = location.replace(router.base, "");

	return (
		<Box px={"7"}>
			<Flex mb={"6"} mt="6">
				<Container>
					<Flex gap="1" align={"baseline"}>
						<Link href="/">
							<Heading
								className={styles.logo}
								size={"7"}
								weight={"bold"}
								mr="5"
							>
								keygate.io
							</Heading>
						</Link>
						<NavItem href="/" active={currentPath === "/"}>
							Dashboard
						</NavItem>
						<NavItem href="/users" active={currentPath === "/users"}>
							Users
						</NavItem>
						<NavItem
							href="/applications"
							active={currentPath === "/applications"}
						>
							Applications
						</NavItem>
						<UserInfo />
					</Flex>
				</Container>
			</Flex>
			<Container>
				<Flex>{children}</Flex>
			</Container>
		</Box>
	);
};

const NavItem = ({
	active,
	children,
	href,
}: { active?: boolean; children: React.ReactNode; href: string }) => (
	<Link href={href}>
		<Button
			mr={"3"}
			highContrast={active}
			color="gray"
			variant="ghost"
			size={"1"}
			className={styles.navItem}
		>
			<Text size={"4"}>{children}</Text>
		</Button>
	</Link>
);

const UserInfo = () => {
	const [, setLocation] = useLocation();
	return (
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				<Button highContrast ml={"auto"} variant="ghost" size={"2"}>
					<Flex align={"center"} gap="3" pl={"2"} py={"1"}>
						<Text size={"2"}>mail@example.com</Text>
						<Avatar fallback={":)"} />
					</Flex>
				</Button>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content
				style={{ minWidth: "9rem" }}
				align="end"
				sideOffset={7}
			>
				<DropdownMenu.Item
					onClick={() => {
						setLocation("/settings");
					}}
				>
					Settings
				</DropdownMenu.Item>
				<DropdownMenu.Separator />
				<DropdownMenu.Item
					onClick={() => {
						setLocation("/auth/login");
					}}
					color="red"
				>
					Sign out
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	);
};
