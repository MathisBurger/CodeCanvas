"use client";
import {
  Button,
  Checkbox,
  Group,
  NumberInput,
  Paper,
  Select,
  Stack,
  TextInput,
} from "@mantine/core";
import { AnswerType, QuestionCatalogueElement } from "@/service/types/tasky";
import { useMemo } from "react";
import {useTranslation} from "react-i18next";

interface QuestionInputProps {
  value: QuestionCatalogueElement;
  setValue: (value: QuestionCatalogueElement) => void;
  remove: () => void;
}

const QuestionInput = ({ value, setValue, remove }: QuestionInputProps) => {

  const {t} = useTranslation(['assignment', 'common']);

  const answerInput = useMemo<JSX.Element | null>(() => {
    switch (value.answer_type) {
      case AnswerType.String:
      case AnswerType.StrContains:
        return (
          <TextInput
            value={value.answer ?? ""}
            onChange={(e) => setValue({ ...value, answer: e.target.value })}
            label={t('fields.answer')}
          />
        );
      case AnswerType.Number:
        return (
          <NumberInput
            value={value.answer ?? 0}
            onChange={(val) => setValue({ ...value, answer: val })}
            label={t('fields.answer')}
          />
        );
      case AnswerType.Boolean:
        return (
          <Checkbox
            checked={value.answer ?? false}
            onChange={(e) => setValue({ ...value, answer: e.target.checked })}
            label={t('fields.answer')}
          />
        );
      default:
        return null;
    }
  }, [value, setValue]);

  return (
    <Paper withBorder p="sm">
      <Stack>
        <Group justify="flex-end">
          <Button onClick={remove} w={100} color="red">
            {t('common:actions.remove')}
          </Button>
        </Group>
        <TextInput
          label={t('fields.question')}
          value={value.question}
          onChange={(e) => setValue({ ...value, question: e.target.value })}
        />
        <Select
          value={value.answer_type}
          onChange={(type) =>
            setValue({ ...value, answer_type: type as AnswerType })
          }
          label={t('fields.answer-type')}
          data={Object.values(AnswerType)}
        />
        {answerInput}
      </Stack>
    </Paper>
  );
};

export default QuestionInput;
