import { useMemo } from "react";
import dayjs from "dayjs";
import { Text } from "@mantine/core";

interface AssignmentDateDisplayProps {
  date: string;
}

const AssignmentDateDisplay = ({ date }: AssignmentDateDisplayProps) => {
  const dueDate = useMemo(() => new Date(date), [date]);
  const dueDateIsOver = useMemo(
    () => dueDate.getTime() < new Date().getTime(),
    [dueDate],
  );
  const formattedDueDate = useMemo(
    () => dayjs(dueDate).format("DD/MM/YYYY hh:mm"),
    [dueDate],
  );

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
