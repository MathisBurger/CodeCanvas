import { Checkbox, NumberInput, Stack, Text, TextInput } from "@mantine/core";
import { AnswerType } from "@/service/types/tasky";
import { useMemo } from "react";

interface AnswerInputProps {
  value: any;
  setValue: (value: any) => void;
  answerType: AnswerType;
  question: string;
}

const AnswerInput = ({
  value,
  setValue,
  answerType,
  question,
}: AnswerInputProps) => {
  const answerInput = useMemo<JSX.Element | null>(() => {
    switch (answerType) {
      case AnswerType.String:
      case AnswerType.StrContains:
        return (
          <TextInput
            value={value}
            onChange={(e) => setValue(e.target.value)}
            label="Answer"
          />
        );
      case AnswerType.Number:
        return (
          <NumberInput
            value={value}
            onChange={(val) => setValue(val)}
            label="Answer"
          />
        );
      case AnswerType.Boolean:
        return (
          <Checkbox
            checked={value}
            onChange={(e) => setValue(e.target.checked)}
            label="Answer"
          />
        );
      default:
        return null;
    }
  }, [value, answerType]);

  return (
    <Stack>
      <Text>{question}</Text>
      {answerInput}
    </Stack>
  );
};

export default AnswerInput;
