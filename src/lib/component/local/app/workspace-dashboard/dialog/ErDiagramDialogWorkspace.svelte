<script lang="ts">
	import type {
		DatabaseConnectionInterface,
		DatasourceInterface
	} from '$lib/model/interface/schema.interface';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import * as d3 from 'd3';

	let { onClose, databaseConnection, datasource, databaseName } = $props<{
		onClose: () => void;
		databaseConnection: DatabaseConnectionInterface;
		datasource: DatasourceInterface[];
		databaseName?: string;
	}>();

	let sqliteErDiagram: any = $state();
	let mysqlErDiagram: any = $state();
	let svgElement: SVGSVGElement;

	onMount(async () => {
		if (!databaseConnection) return;
		try {
			switch (databaseConnection.datasource_id) {
				case 1:
					break;
				case 2:
					sqliteErDiagram = await invoke('get_er_diagram_from_sqlite', {
						url: databaseConnection.url
					});
					if (sqliteErDiagram && svgElement) {
						// Parse the JSON if it's a string
						const diagramData =
							typeof sqliteErDiagram === 'string'
								? JSON.parse(sqliteErDiagram)
								: sqliteErDiagram;
						generateERD(diagramData, svgElement);
					}
					break;
				case 3:
					break;
				case 4:
					mysqlErDiagram = await invoke('get_er_diagram_from_mysql', {
						url: databaseConnection.url,
						database: databaseName
					});

					// Debug the raw data
					console.log('Raw MySQL ER Diagram data:', mysqlErDiagram);

					if (mysqlErDiagram && svgElement) {
						// Parse the JSON if it's a string
						const rawData =
							typeof mysqlErDiagram === 'string'
								? JSON.parse(mysqlErDiagram)
								: mysqlErDiagram;

						// Transform the data to match the expected format
						const diagramData = rawData.map(
							(table: { name: string; columns: any[] }) => ({
								tableName: table.name,
								columns: table.columns.map((col) => ({
									columnName: col.name,
									columnType: col.data_type,
									isNullable: col.is_nullable,
									isPrimaryKey: col.is_primary_key,
									isForeignKey: col.is_foreign_key,
									references: col.references
								}))
							})
						);

						generateERD(diagramData, svgElement);
					}
					break;
				case 5:
					break;
				case 6:
					mysqlErDiagram = await invoke('get_er_diagram_from_mysql', {
						url: databaseConnection.url,
						database: databaseName
					});

					console.log('Raw MySQL/MariaDB ER Diagram data:', mysqlErDiagram);

					if (mysqlErDiagram && svgElement) {
						// Parse the JSON if it's a string
						const rawData =
							typeof mysqlErDiagram === 'string'
								? JSON.parse(mysqlErDiagram)
								: mysqlErDiagram;

						// Transform the data to match the expected format
						const transformedData = rawData.map(
							(table: { name: string; columns: any[] }) => ({
								tableName: table.name,
								columns: table.columns.map((col) => ({
									columnName: col.name,
									columnType: col.data_type,
									isNullable: col.is_nullable,
									isPrimaryKey: col.is_primary_key,
									isForeignKey: col.is_foreign_key,
									references: col.references
								}))
							})
						);

						console.log(
							'Transformed MySQL/MariaDB ER Diagram data:',
							transformedData
						);

						generateERD(transformedData, svgElement);
					}
					break;
				default:
					break;
			}
		} catch (error) {
			console.error('Error fetching ER diagram:', error);
		}
	});

	function handleClose() {
		onClose();
	}

	function generateERD(databaseSchema: any[], svg: SVGSVGElement) {
		const width = 1200;
		const height = 800;
		const margin = { top: 50, right: 50, bottom: 50, left: 50 };
		const innerWidth = width - margin.left - margin.right;
		const innerHeight = height - margin.top - margin.bottom;
		const tableWidth = 300; // Wider tables to prevent text cutoff
		const columnHeight = 25;
		const tablePadding = 50;
		const textPadding = 8;
		const arrowSize = 10;
		const cornerRadius = 5;

		// Create a container for panning and zooming
		const svgSelection = d3
			.select(svg)
			.attr('width', '100%')
			.attr('height', '100%')
			.attr('viewBox', `0 0 ${width} ${height}`)
			.attr('preserveAspectRatio', 'xMidYMid meet');

		// Clear any existing content
		svgSelection.selectAll('*').remove();

		// Create a container for all diagram elements
		const container = svgSelection.append('g').attr('class', 'container');

		// Organize tables into 3 rows with dynamic columns
		const numTables = databaseSchema.length;
		const tablesPerRow = Math.ceil(numTables / 3);

		const tablesWithPosition = databaseSchema.map((table, index) => {
			const row = Math.floor(index / tablesPerRow);
			const col = index % tablesPerRow;

			return {
				...table,
				index,
				x: margin.left + col * (tableWidth + tablePadding),
				y: margin.top + row * 300, // Fixed row height for better spacing
				isDragging: false
			};
		});

		function getTableHeight(table: any) {
			return 30 + table.columns.length * columnHeight;
		}

		// Extract foreign key relationships
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

		// Function to update relationship arrows
		function updateRelationships() {
			linkGroups.selectAll('path').attr('d', (d: any) => {
				const sourceTable = tablesWithPosition.find(
					(t) => t.tableName === d.sourceTable
				);
				const targetTable = tablesWithPosition.find(
					(t) => t.tableName === d.targetTable
				);

				if (!sourceTable || !targetTable) return '';

				// Determine the best sides to connect based on relative positions
				let sourceX, sourceY, targetX, targetY;

				// Calculate centers
				const sourceCenterX = sourceTable.x + tableWidth / 2;
				const sourceCenterY = sourceTable.y + getTableHeight(sourceTable) / 2;
				const targetCenterX = targetTable.x + tableWidth / 2;
				const targetCenterY = targetTable.y + getTableHeight(targetTable) / 2;

				// Determine which sides to connect
				if (
					Math.abs(sourceCenterX - targetCenterX) >
					Math.abs(sourceCenterY - targetCenterY)
				) {
					// Connect left/right sides
					if (sourceCenterX < targetCenterX) {
						// Source is to the left of target
						sourceX = sourceTable.x + tableWidth;
						targetX = targetTable.x;
					} else {
						// Source is to the right of target
						sourceX = sourceTable.x;
						targetX = targetTable.x + tableWidth;
					}

					// Y positions for the specific columns
					sourceY =
						sourceTable.y +
						30 +
						sourceTable.columns.findIndex(
							(c: any) => c.columnName === d.sourceColumn
						) *
							columnHeight +
						columnHeight / 2;

					// Find the primary key column in target table
					const pkIndex = targetTable.columns.findIndex(
						(c: any) => c.isPrimaryKey
					);
					targetY =
						targetTable.y +
						30 +
						(pkIndex !== -1 ? pkIndex : 0) * columnHeight +
						columnHeight / 2;
				} else {
					// Connect top/bottom sides
					if (sourceCenterY < targetCenterY) {
						// Source is above target
						sourceY = sourceTable.y + getTableHeight(sourceTable);
						targetY = targetTable.y;
					} else {
						// Source is below target
						sourceY = sourceTable.y;
						targetY = targetTable.y + getTableHeight(targetTable);
					}

					// X positions at the middle of the tables
					sourceX = sourceTable.x + tableWidth / 2;
					targetX = targetTable.x + tableWidth / 2;
				}

				// Create a curved path
				const path = d3.path();
				path.moveTo(sourceX, sourceY);

				// Calculate control points for a smooth curve
				const dx = targetX - sourceX;
				const dy = targetY - sourceY;
				const midX = sourceX + dx / 2;
				const midY = sourceY + dy / 2;

				// Use a quadratic curve for smoother appearance
				path.quadraticCurveTo(midX, midY, targetX, targetY);

				return path.toString();
			});
		}

		// Set up zoom behavior
		let activeTableDrag = false;
		const zoom = d3
			.zoom()
			.scaleExtent([0.3, 3])
			.filter((event) => {
				// Allow zooming only when no table is being dragged
				return !activeTableDrag && !event.button;
			})
			.on('zoom', (event) => {
				container.attr('transform', event.transform);
			});

		// Apply zoom behavior to SVG
		svgSelection.call(zoom as any);

		// Draw tables
		const tableGroups = container
			.selectAll('.table-group')
			.data(tablesWithPosition)
			.join('g')
			.attr('class', 'table-group')
			.attr('transform', (d) => `translate(${d.x}, ${d.y})`)
			.style('cursor', 'move');

		// Set up drag behavior for tables
		const tableDrag = d3
			.drag()
			.subject(function (event, d) {
				// Return the current position to make dragging smoother
				return {
					x: (d as { x: number; y: number }).x,
					y: (d as { x: number; y: number }).y
				};
			})
			.on('start', function (event, d) {
				// Set flag to prevent zoom during drag
				activeTableDrag = true;
				d3.select(this).raise(); // Bring to front
				d3.select(this).attr('data-dragging', 'true');
			})
			.on('drag', function (event, d) {
				// Update the table position
				(d as { x: number }).x = event.x;
				(d as { y: number }).y = event.y;
				d3.select(this).attr(
					'transform',
					`translate(${(d as { x: number }).x}, ${(d as { y: number }).y})`
				);

				// Update relationship arrows
				updateRelationships();
			})
			.on('end', function (event, d) {
				// Reset flag to allow zoom again
				activeTableDrag = false;
				d3.select(this).attr('data-dragging', null);
			});

		// Apply drag behavior to tables
		tableGroups.call(tableDrag as any);

		// Table background
		tableGroups
			.append('rect')
			.attr('class', 'table-bg')
			.attr('width', tableWidth)
			.attr('height', (d) => getTableHeight(d))
			.attr('fill', '#f9f9f9')
			.attr('stroke', '#ccc')
			.attr('stroke-width', 1)
			.attr('rx', cornerRadius)
			.attr('ry', cornerRadius);

		// Table header background
		tableGroups
			.append('rect')
			.attr('class', 'table-header-bg')
			.attr('width', tableWidth)
			.attr('height', 30) // Fixed header height
			.attr('fill', '#e6e6e6')
			.attr('stroke', '#ccc')
			.attr('stroke-width', 1)
			.attr('rx', cornerRadius)
			.attr('ry', 0);

		// Table name
		tableGroups
			.append('text')
			.attr('class', 'table-name')
			.attr('x', textPadding)
			.attr('y', 20)
			.attr('font-size', '14px')
			.attr('font-weight', 'bold')
			.attr('fill', '#333')
			.text((d) => d.tableName);

		// Column groups
		const columnGroups = tableGroups
			.selectAll('.column-group')
			.data((d) => d.columns)
			.join('g')
			.attr('class', 'column-group')
			.attr('transform', (d, i) => `translate(0, ${30 + i * columnHeight})`);

		// Column background for better readability
		columnGroups
			.append('rect')
			.attr('class', 'column-bg')
			.attr('width', tableWidth)
			.attr('height', columnHeight)
			.attr('fill', (d, i) => (i % 2 === 0 ? '#f5f5f5' : '#ffffff'))
			.attr('opacity', 0.7);

		// Column text
		columnGroups
			.append('text')
			.attr('class', 'column-text')
			.attr('x', textPadding)
			.attr('y', columnHeight / 2 + 5) // Better vertical centering
			.attr('font-size', '12px')
			.attr('fill', '#555')
			.text(
				(d: any) =>
					`${d.columnName}: ${d.columnType}${d.isPrimaryKey ? ' (PK)' : ''}${d.isForeignKey ? ' (FK)' : ''}`
			);

		// Draw foreign key relationships (arrows)
		const linkGroups = container
			.selectAll('.link-group')
			.data(foreignKeys)
			.join('g')
			.attr('class', 'link-group');

		linkGroups
			.append('path')
			.attr('class', 'link')
			.attr('fill', 'none')
			.attr('stroke', '#666')
			.attr('stroke-width', 1.5)
			.attr('marker-end', 'url(#arrow)');

		// Add arrow marker definition
		svgSelection
			.append('defs')
			.append('marker')
			.attr('id', 'arrow')
			.attr('viewBox', '0 -5 10 10')
			.attr('refX', 8)
			.attr('refY', 0)
			.attr('markerWidth', arrowSize)
			.attr('markerHeight', arrowSize)
			.attr('orient', 'auto')
			.append('path')
			.attr('d', 'M0,-5L10,0L0,5')
			.attr('fill', '#666');

		// Initial update of relationships
		updateRelationships();

		// Make the SVG scrollable if it's too large
		const totalHeight = tablesWithPosition.reduce(
			(max, table) =>
				Math.max(max, table.y + getTableHeight(table) + margin.bottom),
			0
		);
		const totalWidth = tablesWithPosition.reduce(
			(max, table) => Math.max(max, table.x + tableWidth + margin.right),
			0
		);

		// Set initial zoom to fit all content
		const initialScale = Math.min(
			width / totalWidth,
			height / totalHeight,
			0.9 // Slightly zoom out to show everything
		);

		svgSelection.call(
			(zoom as any).transform,
			d3.zoomIdentity
				.translate(width / 2, height / 2)
				.scale(initialScale)
				.translate(-totalWidth / 2, -totalHeight / 2)
		);

		// Function to reset view
		(window as any).resetERDiagramView = () => {
			svgSelection
				.transition()
				.duration(750)
				.call(
					(zoom as any).transform,
					d3.zoomIdentity
						.translate(width / 2, height / 2)
						.scale(initialScale)
						.translate(-totalWidth / 2, -totalHeight / 2)
				);
		};
	}
</script>

<div class="modal modal-open">
	<div class="modal-box flex h-[90vh] w-11/12 max-w-5xl flex-col gap-3">
		<h2 class="text-2xl font-bold">ER Diagram</h2>
		<p class="text-sm text-gray-500">
			Drag tables to reposition them. Use mouse wheel or trackpad to zoom
			in/out.
		</p>

		<div class="border-base-300 flex-1 overflow-hidden rounded-lg border p-2">
			{#if datasource && databaseConnection}
				{#each datasource as source}
					{#if databaseConnection.datasource_id === source.id}
						{#if source.name === 'SQLite' || source.name === 'MySQL' || source.name === 'MariaDB'}
							<div class="h-full w-full">
								<svg bind:this={svgElement} class="h-full w-full"></svg>
							</div>
						{:else}
							<div class="flex h-full items-center justify-center">
								<p class="text-lg">
									ER Diagram is only available for SQLite, MySQL and MariaDB
									databases
								</p>
							</div>
						{/if}
					{/if}
				{/each}
			{:else}
				<div class="flex h-full items-center justify-center">
					<p class="text-lg">No database connection selected</p>
				</div>
			{/if}
		</div>

		<div class="modal-action">
			<button
				class="btn btn-primary"
				onclick={() => {
					if ((window as any).resetERDiagramView) {
						(window as any).resetERDiagramView();
					}
				}}>Reset View</button
			>
			<button class="btn btn-error" onclick={handleClose}>Close</button>
		</div>
	</div>
</div>

<style>
	/* Add some styles to improve the dragging experience */
	:global(.table-group[data-dragging='true']) {
		opacity: 0.9;
	}

	:global(.table-group[data-dragging='true'] .table-bg) {
		filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.2));
	}

	:global(.table-group:hover .table-header-bg) {
		fill: #d9d9d9;
	}
</style>
