import { Badge } from "@mantine/core";

interface SolutionBadgeProps {
  status: string | null;
}

const SolutionBadge = ({ status }: SolutionBadgeProps): JSX.Element => {
  switch (status) {
    case "APPROVED":
      return <Badge color="green">{status}</Badge>;
    case "REJECTED":
      return <Badge color="red">{status}</Badge>;
    case "SUCCESSFUL":
      return <Badge color="gray">{status}</Badge>;
    case "FAILED":
      return <Badge color="gray">{status}</Badge>;
    default:
      return <Badge color="yellow">{status ?? "PENDING"}</Badge>;
  }
};

export default SolutionBadge;
