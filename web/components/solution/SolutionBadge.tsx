import { Badge } from "@mantine/core";
import { Job } from "@/service/types/tasky";

interface SolutionBadgeProps {
  status: string | null;
  job?: Job;
}

const SolutionBadge = ({ status, job }: SolutionBadgeProps): JSX.Element => {
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
      if (job !== null && job?.execution[0].error) {
        return <Badge color="gray">Failed</Badge>;
      }
      if (job !== null && job?.execution[0].result) {
        return <Badge color="gray">Successful</Badge>;
      }
      return <Badge color="yellow">{status ?? "PENDING"}</Badge>;
  }
};

export default SolutionBadge;
