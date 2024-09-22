'use client';
import {Box, Group, Image} from "@mantine/core"
import SsrHeader from "@/components/SsrHeader";


const Header = async () => {
    //const headerList = headers();
    //const pathname = headerList.get("x-current-path");
    //const api = useApiService();
    let user = null;

    /*try {
        user = await api.self() as User;
    } catch (e) {
        if (pathname !== "/login" && pathname !== "/register" && pathname !== "/") {
            return <RedirectComponent to="/login" />;
        }
    }*/

    return (
        <Box pr={20}>
            <header>
                <Group justify="space-between" h="100%">
                    <Image src="/CodeCanvas.png" h={100} alt="CompanyLogo" />
                </Group>
            </header>
        </Box>
    );
}

export default Header;
