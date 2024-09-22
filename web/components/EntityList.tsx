'use client';
import {Button, ButtonProps, Group, Table} from "@mantine/core";


export interface EntityListCol {
    label: string;
    field: string;
    getter?: (row: any) => string|number;
}

export interface EntityListRowAction {
    color: ButtonProps['color'];
    name: string;
    onClick: (row: any) => void;
}

interface EntityListProps {
    cols: EntityListCol[];
    rows: any[];
    rowActions?: EntityListRowAction[];
}

const EntityList: React.FC<EntityListProps> = ({cols, rows, rowActions}) => {

    return (
        <Table stickyHeader>
            <Table.Thead>
                {cols.map(col => (
                    <Table.Th key={col.label}>{col.label}</Table.Th>
                ))}
                {rowActions && (
                    <Table.Th>Actions</Table.Th>
                )}
            </Table.Thead>
            <Table.Tbody>
                {rows.map(row => (
                    <Table.Tr key={`${row}`}>
                        {cols.map(col => (
                            <Table.Td key={`${row}_${col}`}>{col.getter ? col.getter(row) : row[col.field]}</Table.Td>
                        ))}
                        {rowActions && (
                            <Table.Td>
                                <Group justify="center">
                                    {rowActions.map(action => (
                                        <Button
                                            onClick={() => action.onClick(row)}
                                            color={action.color}
                                            key={`${row}_${action.name}`}
                                        >{action.name}</Button>
                                    ))}
                                </Group>
                            </Table.Td>
                        )}
                    </Table.Tr>
                ))}
            </Table.Tbody>
        </Table>
    );
}

export default EntityList;
