"use client";

import {
  Combobox,
  Container,
  Input,
  InputBase,
  MantineColorScheme,
  Title,
  useCombobox,
  useMantineColorScheme,
} from "@mantine/core";
import { useTranslation } from "react-i18next";

const schemes = ["light", "dark", "auto"];

const SettingsPage = () => {
  const { colorScheme, setColorScheme } = useMantineColorScheme({
    keepTransitions: true,
  });
  const combobox = useCombobox({
    onDropdownClose: () => combobox.resetSelectedOption(),
  });
  const { t } = useTranslation("common");

  const options = schemes.map((item) => (
    <Combobox.Option value={item} key={item}>
      {item}
    </Combobox.Option>
  ));

  return (
    <Container fluid>
      <Title>{t("settings.settings")}</Title>
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
    </Container>
  );
};

export default SettingsPage;
