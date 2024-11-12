import { Badge } from "@mantine/core";
import { useTranslation } from "react-i18next";

interface SolutionBadgeProps {
  status: string | null;
}

const SolutionBadge = ({ status }: SolutionBadgeProps): JSX.Element => {
  const { t } = useTranslation("solution");

  switch (status) {
    case "APPROVED":
      return <Badge color="green">{t("status.approved")}</Badge>;
    case "REJECTED":
      return <Badge color="red">{t("status.rejected")}</Badge>;
    case "SUCCESSFUL":
      return <Badge color="gray">{t("status.successful")}</Badge>;
    case "FAILED":
      return <Badge color="gray">{t("status.failed")}</Badge>;
    default:
      return <Badge color="yellow">{t("status.pending")}</Badge>;
  }
};

export default SolutionBadge;
