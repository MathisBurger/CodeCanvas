"use client";

import {
  Button,
  Combobox,
  Container,
  Input,
  InputBase,
  MantineColorScheme,
  Stack,
  Title,
  useCombobox,
  useMantineColorScheme,
} from "@mantine/core";
import {useTranslation} from "react-i18next";
import useCurrentUser from "@/hooks/useCurrentUser";
import {isGranted} from "@/service/auth";
import {UserRoles} from "@/service/types/usernator";
import {useState} from "react";
import SwitchToTutorModal from "@/components/SwitchToTutorModal";

const schemes = ["light", "dark", "auto"];

const SettingsPage = () => {
  const { colorScheme, setColorScheme } = useMantineColorScheme({
    keepTransitions: true,
  });
  const combobox = useCombobox({
    onDropdownClose: () => combobox.resetSelectedOption(),
  });
  const { t } = useTranslation("common");
  const {user} = useCurrentUser();
  const [switchModalOpen, setSwitchModalOpen] = useState<boolean>(false);

  const options = schemes.map((item) => (
    <Combobox.Option value={item} key={item}>
      {item}
    </Combobox.Option>
  ));

  return (
    <Container fluid>
      <Title>{t("settings.settings")}</Title>
      <Stack gap={25} mt={10}>
        <Combobox
            store={combobox}
            withinPortal={false}
            onOptionSubmit={(val) => {
              setColorScheme(val as MantineColorScheme);
            }}
        >
          <Combobox.Target>
            <InputBase
                component="button"
                type="button"
                pointer
                rightSection={<Combobox.Chevron />}
                onClick={() => combobox.toggleDropdown()}
                rightSectionPointerEvents="none"
            >
              {colorScheme || <Input.Placeholder>Pick value</Input.Placeholder>}
            </InputBase>
          </Combobox.Target>

          <Combobox.Dropdown>
            <Combobox.Options>{options}</Combobox.Options>
          </Combobox.Dropdown>
        </Combobox>
        {isGranted(user, [UserRoles.Student]) && (
            <Button variant="gradient" w="25%" onClick={() => setSwitchModalOpen(true)}>{t('common:titles.switch-to-tutor')}</Button>
        )}
      </Stack>
      {switchModalOpen && (
          <SwitchToTutorModal onClose={() => setSwitchModalOpen(false)} />
      )}
    </Container>
  );
};

export default SettingsPage;
