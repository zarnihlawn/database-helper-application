<script lang="ts">
	import * as d3 from 'd3';
	import { onMount } from 'svelte';

	const data = [
		{
			tableName: 'users',
			columns: [
				{ columnName: 'id', columnType: 'INT', isPrimaryKey: true },
				{
					columnName: 'username',
					columnType: 'VARCHAR(50)',
					isNullable: false
				},
				{ columnName: 'email', columnType: 'VARCHAR(100)', isNullable: false },
				{
					columnName: 'created_at',
					columnType: 'TIMESTAMP',
					default: 'CURRENT_TIMESTAMP'
				}
			]
		},
		{
			tableName: 'orders',
			columns: [
				{ columnName: 'order_id', columnType: 'INT', isPrimaryKey: true },
				{
					columnName: 'user_id',
					columnType: 'INT',
					isForeignKey: true,
					references: 'users(id)'
				},
				{ columnName: 'order_date', columnType: 'DATE', isNullable: false },
				{
					columnName: 'total_amount',
					columnType: 'DECIMAL(10, 2)',
					isNullable: false
				}
			]
		},
		{
			tableName: 'products',
			columns: [
				{ columnName: 'product_id', columnType: 'INT', isPrimaryKey: true },
				{ columnName: 'name', columnType: 'VARCHAR(255)', isNullable: false },
				{ columnName: 'price', columnType: 'DECIMAL(8, 2)', isNullable: false }
			]
		},
		{
			tableName: 'order_items',
			columns: [
				{ columnName: 'item_id', columnType: 'INT', isPrimaryKey: true },
				{
					columnName: 'order_id',
					columnType: 'INT',
					isForeignKey: true,
					references: 'orders(order_id)'
				},
				{
					columnName: 'product_id',
					columnType: 'INT',
					isForeignKey: true,
					references: 'products(product_id)'
				},
				{ columnName: 'quantity', columnType: 'INT', isNullable: false },
				{
					columnName: 'unit_price',
					columnType: 'DECIMAL(8, 2)',
					isNullable: false
				}
			]
		}
	];

	let svgElement: SVGSVGElement | undefined;

	onMount(() => {
		if (svgElement) {
			generateERD(data, svgElement);
		}
	});

	function generateERD(databaseSchema: any[], svg: SVGSVGElement) {
		const width = 800;
		const height = 600;
		const margin = { top: 50, right: 50, bottom: 50, left: 50 };
		const innerWidth = width - margin.left - margin.right;
		const innerHeight = height - margin.top - margin.bottom;
		const tableWidth = 180;
		const columnHeight = 25;
		const tablePadding = 30;
		const textPadding = 8;
		const arrowSize = 10;
		const cornerRadius = 5;

		const svgSelection = d3
			.select(svg)
			.attr('width', width)
			.attr('height', height);

		// Calculate table positions using a simple grid layout
		const tablesPerRow = Math.floor(innerWidth / (tableWidth + tablePadding));
		const tablesWithPosition = databaseSchema.map((table, index) => ({
			...table,
			x: margin.left + (index % tablesPerRow) * (tableWidth + tablePadding),
			y:
				margin.top +
				Math.floor(index / tablesPerRow) *
					(getTableHeight(table) + tablePadding * 2)
		}));

		function getTableHeight(table: any) {
			return 20 + table.columns.length * columnHeight;
		}

		const foreignKeys: any[] = [];
		tablesWithPosition.forEach((table) => {
			table.columns.forEach((column: any) => {
				if (column.isForeignKey && column.references) {
					const [targetTableName] = column.references.split('(');
					const sourceTable = table.tableName;
					foreignKeys.push({
						sourceTable,
						sourceColumn: column.columnName,
						targetTable: targetTableName
					});
				}
			});
		});

		// Draw tables
		const tableGroups = svgSelection
			.selectAll('.table-group')
			.data(tablesWithPosition)
			.join('g')
			.attr('class', 'table-group')
			.attr('transform', (d) => `translate(${d.x}, ${d.y})`);

		tableGroups
			.append('rect')
			.attr('width', tableWidth)
			.attr('height', (d) => getTableHeight(d))
			.attr('fill', '#f9f9f9')
			.attr('stroke', '#ccc')
			.attr('stroke-width', 1)
			.attr('rx', cornerRadius)
			.attr('ry', cornerRadius)
			.style('box-shadow', '2px 2px 5px rgba(0, 0, 0, 0.1)');

		tableGroups
			.append('text')
			.attr('x', textPadding)
			.attr('y', 20)
			.attr('font-size', '14px')
			.attr('font-weight', 'bold')
			.attr('fill', '#333')
			.text((d) => d.tableName);

		const columnGroups = tableGroups
			.selectAll('.column-group')
			.data((d) => d.columns)
			.join('g')
			.attr('class', 'column-group')
			.attr('transform', (d, i) => `translate(0, ${20 + i * columnHeight})`);

		columnGroups
			.append('text')
			.attr('x', textPadding)
			.attr('y', columnHeight / 2 + textPadding)
			.attr('font-size', '12px')
			.attr('fill', '#555')
			.text(
				(d: any) =>
					`${d.columnName}: ${d.columnType}${d.isPrimaryKey ? ' (PK)' : ''}${d.isForeignKey ? ' (FK)' : ''}`
			);

		// Draw foreign key relationships (arrows)
		const linkGroups = svgSelection
			.selectAll('.link-group')
			.data(foreignKeys)
			.join('g')
			.attr('class', 'link-group');

		linkGroups
			.append('path')
			.attr('class', 'link')
			.attr('d', (d) => {
				const sourceTable = tablesWithPosition.find(
					(t) => t.tableName === d.sourceTable
				);
				const targetTable = tablesWithPosition.find(
					(t) => t.tableName === d.targetTable
				);

				if (!sourceTable || !targetTable) return '';

				const sourceX = sourceTable.x + tableWidth / 2;
				const sourceY =
					sourceTable.y +
					20 +
					sourceTable.columns.findIndex(
						(c: any) => c.columnName === d.sourceColumn
					) *
						columnHeight +
					columnHeight / 2;
				const targetX = targetTable.x + tableWidth / 2;
				const targetY =
					targetTable.y +
					20 +
					targetTable.columns.findIndex(
						(c: any) => c.columnName === d.targetColumn
					) *
						columnHeight +
					columnHeight / 2;

				const deltaX = targetX - sourceX;
				const deltaY = targetY - sourceY;
				const path = d3.path();
				path.moveTo(sourceX, sourceY);

				// Add a slight curve for better visual appeal
				const midX = (sourceX + targetX) / 2;
				const controlPointOffset = 20;
				path.bezierCurveTo(
					midX - controlPointOffset,
					sourceY,
					midX + controlPointOffset,
					targetY,
					targetX,
					targetY
				);

				return path.toString();
			})
			.attr('fill', 'none')
			.attr('stroke', '#aaa')
			.attr('stroke-width', 1);

		// Add arrowheads
		svgSelection
			.append('defs')
			.append('marker')
			.attr('id', 'arrowhead')
			.attr('viewBox', '0 -5 10 10')
			.attr('refX', arrowSize)
			.attr('refY', 0)
			.attr('orient', 'auto')
			.attr('markerWidth', arrowSize)
			.attr('markerHeight', arrowSize)
			.attr('fill', '#aaa')
			.append('path')
			.attr('d', 'M0,-5L10,0L0,5');

		svgSelection.selectAll('.link').attr('marker-end', 'url(#arrowhead)');
	}
</script>

<h1>Database ERD</h1>

<div style="overflow-x: auto;">
	<svg bind:this={svgElement}></svg>
</div>

<style>
	svg {
		display: block; /* Prevent extra space below the SVG */
	}
</style>
