"use client";
import { useRouter } from "next/navigation";
import { Button, Group } from "@mantine/core";
import {useTranslation} from "react-i18next";

const NavigateBack = () => {
  const router = useRouter();
  const {t} = useTranslation('common');

  return (
    <Group justify="start" mt={30}>
      <Button color="blue" onClick={() => router.back()}>
          {t('actions.navigate-back')}
      </Button>
    </Group>
  );
};

export default NavigateBack;
