import { useMemo } from "react";
import dayjs from "dayjs";
import { Text } from "@mantine/core";

interface AssignmentDateDisplayProps {
  date: string | null;
}

const AssignmentDateDisplay = ({ date }: AssignmentDateDisplayProps) => {
  const dueDate = useMemo(() => (date ? new Date(date) : null), [date]);
  const dueDateIsOver = useMemo(
    () => (dueDate ? dueDate.getTime() < new Date().getTime() : null),
    [dueDate],
  );
  const formattedDueDate = useMemo(
    () => (dueDate ? dayjs(dueDate).format("DD/MM/YYYY hh:mm") : null),
    [dueDate],
  );

  if (formattedDueDate === null) {
    return null;
  }

  return (
    <Text
      c={dueDateIsOver ? "red" : "gray"}
      td={dueDateIsOver ? "line-through" : undefined}
    >
      {formattedDueDate}
    </Text>
  );
};

export default AssignmentDateDisplay;
