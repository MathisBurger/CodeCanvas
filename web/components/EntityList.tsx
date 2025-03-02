"use client";
import { Button, ButtonProps, Group, Table } from "@mantine/core";
import { UserRoles } from "@/service/types/usernator";
import { useMemo } from "react";
import { isGranted } from "@/service/auth";
import useCurrentUser from "@/hooks/useCurrentUser";
import { useTranslation } from "react-i18next";
import CryptoJS from "crypto-js";

export interface EntityListCol {
  label: string;
  field: string;
  getter?: (row: any) => string | number;
  render?: (value: string | number, row: any) => JSX.Element;
}

export interface EntityListRowAction {
  color: ButtonProps["color"];
  name: string;
  onClick: (row: any) => void;
  auth?: UserRoles[];
  authFunc?: (row: any) => boolean;
}

interface EntityListProps {
  cols: EntityListCol[];
  rows: any[];
  rowActions?: EntityListRowAction[];
}

const EntityList: React.FC<EntityListProps> = ({ cols, rows, rowActions }) => {
  const { user } = useCurrentUser();
  const { t } = useTranslation("common");
  const filteredRowActions = useMemo<undefined | EntityListRowAction[]>(() => {
    if (rowActions) {
      return rowActions.filter((a) =>
        a.auth ? isGranted(user, a.auth) : true,
      );
    }
    return undefined;
  }, [rowActions, user]);

  const getCellValue = (row: any, col: EntityListCol) => {
    const value = col.getter ? col.getter(row) : row[col.field];
    return col.render ? col.render(value, row) : value;
  };

  const hashObj = (obj: object): string => {
    const jsonString = JSON.stringify(obj);
    return CryptoJS.SHA256(jsonString).toString(CryptoJS.enc.Hex);
  };

  return (
    <Table stickyHeader>
      <Table.Thead>
        <Table.Tr>
          {cols.map((col) => (
            <Table.Th key={col.label}>{col.label}</Table.Th>
          ))}
          {filteredRowActions && <Table.Th>{t("cols.actions")}</Table.Th>}
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {rows.map((row) => (
          <Table.Tr key={`${hashObj(row)}`}>
            {cols.map((col) => (
              <Table.Td key={`${hashObj(row)}_${col.field}`}>
                {getCellValue(row, col)}
              </Table.Td>
            ))}
            {filteredRowActions && (
              <Table.Td>
                <Group justify="center">
                  {filteredRowActions
                    .filter((a) => (a.authFunc ? a.authFunc(row) : true))
                    .map((action) => (
                      <Button
                        onClick={() => action.onClick(row)}
                        color={action.color}
                        key={`${row}_${action.name}`}
                      >
                        {action.name}
                      </Button>
                    ))}
                </Group>
              </Table.Td>
            )}
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  );
};

export default EntityList;
