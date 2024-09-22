import {Table} from "@mantine/core";


export interface EntityListCol {
    label: string;
    field: string;
    getter?: (row: any) => string|number;
}

interface EntityListProps {
    cols: EntityListCol[];
    rows: any[];
}

const EntityList: React.FC<EntityListProps> = ({cols, rows}) => {

    return (
        <Table stickyHeader>
            <Table.Thead>
                {cols.map(col => (
                    <Table.Th key={col.label}>{col.label}</Table.Th>
                ))}
            </Table.Thead>
            <Table.Tbody>
                {rows.map(row => (
                    <Table.Tr key={`${row}`}>
                        {cols.map(col => (
                            <Table.Td key={`${row}_${col}`}>{col.getter ? col.getter(row) : row[col.field]}</Table.Td>
                        ))}
                    </Table.Tr>
                ))}
            </Table.Tbody>
        </Table>
    );
}

export default EntityList;
