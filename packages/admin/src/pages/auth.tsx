import { Link, Route, Router, Switch, useLocation, useRouter } from "wouter";
import { Login, Signup } from "@keygate/react";
import { Box, Button, Flex } from "@radix-ui/themes";

import noise from "../../assets/noise.svg";
import { useDarkMode } from "@keygate/react/utils/use-dark-mode";

const bg2 =
	"radial-gradient(77.73% 77.73% at 98.52% 96.25%, rgb(214, 213, 219) 0%, rgba(170, 167, 177, 0.29) 50.52%, rgba(91, 216, 216, 0) 100%), radial-gradient(141.73% 105.23% at 50% -7.16%, rgb(225, 248, 255) 0%, rgba(160, 198, 255, 0) 50.73%, rgba(162, 147, 255, 0) 100%), radial-gradient(112.27% 48.54% at 1.59% 50%, rgba(72, 241, 178, 0.09) 0%, rgba(255, 123, 234, 0) 53.91%, rgba(254, 216, 255, 0) 100%), linear-gradient(153.07deg, rgba(110, 184, 36, 0.14) 6.37%, rgba(255, 230, 166, 0) 83.82%)";
const bg1 = `url(${noise}), radial-gradient(49.19% 88.28% at 72.99% 113.54%, rgba(93, 227, 236, 0.73) 0%, rgba(93, 227, 236, 0.1679) 59.78%, rgb(50, 103, 107) 100%), linear-gradient(54.57deg, #862AB1 8.59%, rgba(185, 91, 230, 0) 80.49%), radial-gradient(118.75% 118.75% at 96.65% 74.22%, #308EE6 0%, rgba(48, 158, 230, 0) 73.91%), radial-gradient(114.51% 155.86% at 9.82% 10.94%, rgb(26, 22, 22) 0%, rgba(255, 85, 85, 0.147) 61.98%, rgba(255, 85, 85, 0) 100%, #F2F5FA29)`;

export const AuthRoutes = () => {
	const router = useRouter();
	const [location] = useLocation();
	const { isDarkMode, toggle } = useDarkMode();

	return (
		<Router base="/auth" parent={router}>
			<Flex
				justify={"center"}
				direction={"column"}
				p="5"
				height={"100%"}
				style={{
					background: isDarkMode ? bg1 : bg2,
					backgroundBlendMode: "color-burn",
				}}
			>
				<Box style={{ position: "absolute", bottom: 0, right: 0 }} p="3">
					<Button variant="ghost" m="2" onClick={toggle}>
						toggle dark mode
					</Button>
				</Box>
				<Flex justify={"end"}>
					<Link href={location.includes("/login") ? "/signup" : "/login"}>
						<Button variant="soft" size={"3"}>
							{location.includes("/login") ? "Sign up" : "Sign in"}
						</Button>
					</Link>
				</Flex>
				<Flex grow={"1"} justify={"center"} align={"center"}>
					<Switch>
						<Route path="/login">
							<Login />
						</Route>
						<Route path="/signup" component={Signup} />
					</Switch>
				</Flex>
			</Flex>
		</Router>
	);
};
