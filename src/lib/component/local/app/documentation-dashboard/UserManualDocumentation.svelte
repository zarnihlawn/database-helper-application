<script lang="ts">
	import DockerStartRestartClick from '$lib/asset/image/webp/docker-start-restart-click.webp';
	import DockerStopClick from '$lib/asset/image/webp/docker-stop-click.webp';
	import DockerDashboardTabs from '$lib/asset/image/webp/docker-dashboard-tabs.webp';
	import DockerContainer from '$lib/asset/image/webp/docker-container-tab.webp';
	import DockerImage from '$lib/asset/image/webp/docker-image-tab.webp';
	import DockerPullingImage from '$lib/asset/image/webp/docker-pull-image.webp';
	import DockerTerminal from '$lib/asset/image/webp/docker-terminal.webp';
	import { PostgreSQLSvg } from '$lib/asset/image/svg/postgresql-svg';
	import { MongoDBSvg } from '$lib/asset/image/svg/mongodb-svg';
	import { MySQLSvg } from '$lib/asset/image/svg/mysql-svg';
	import { SQLiteSvg } from '$lib/asset/image/svg/sqlite-svg';
	import { MariaDBSvg } from '$lib/asset/image/svg/mariadb-svg';
	import { MicrosoftSqlServerSvg } from '$lib/asset/image/svg/microsoft-sql-server-svg';
	import WorkspaceActionMenu from '$lib/asset/image/webp/workspace-action-menu.webp';
	import WorkspaceSelectDatabase from '$lib/asset/image/webp/workspace-select-database.webp';
	import WorkspaceConnectDatabase from '$lib/asset/image/webp/workspace-connect-database.webp';
	import WorkspaceSelectRemoveDatabase from '$lib/asset/image/webp/workspace-select-remove-database.webp';
	import WorkspaceRemoveDatabase from '$lib/asset/image/webp/workspace-remove-database.webp';
	import WorkspaceFileTree from '$lib/asset/image/webp/workspace-file-tree.webp';
	import WorkspaceERDiagram from '$lib/asset/image/webp/workspace-er-diagram.webp';
	import WorkspaceEditor from '$lib/asset/image/webp/workspace-editor.webp';
	import WorkspaceShareDatabase from '$lib/asset/image/webp/workspace-share-database.webp';
</script>

<article
	class="animate-fade-in-up prose prose-xs prose-img:rounded-xl lg:prose-sm !max-w-none"
>
	<h1 class="text-primary"># User Manual</h1>
	<p>
		This is the dedicated page for the user manual of the Database Helper
		Application.
	</p>

	<h3 class="text-primary">## Docker Dashboard</h3>
	<p>
		The Docker Dashboard is a user-friendly interface that allows you to manage
		your Docker containers and images. It provides a simple and intuitive way to
		create, start, stop, and manage Docker containers and images.
	</p>

	<ol>
		<li>Install Docker</li>
		<ul>
			<li>
				To install Docker, you can follow the instructions provided by the
				Docker official website. You can find the installation guide here:
				<a
					href="https://docs.docker.com/get-docker/"
					class="text-blue-600 underline">https://docs.docker.com/get-docker/</a
				>
			</li>
		</ul>
		<li>
			Check Docker Installation
			<ul>
				<li>
					To check if Docker is installed, open a terminal and run the following
					command:
				</li>
				<li>
					<code>docker --version</code>
				</li>
			</ul>
		</li>
		<li>Start | Stop | Restart Docker</li>
		<p>
			Note that only one of the Start or Stop button will appear base on the
			current Docker Service status.
		</p>
		<ul>
			<li>
				To Start or Restart Docker, click these two button accordingly.
				<figure>
					<img
						class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
						src={DockerStartRestartClick}
						alt="Docker Start Restart Click"
					/>
				</figure>
			</li>
			<li>
				To Stop Docker, click the Stop button.
				<figure>
					<img
						class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
						src={DockerStopClick}
						alt="Docker Stop Click"
					/>
				</figure>
			</li>
		</ul>

		<li>Docker Play Ground Tab</li>
		<p>
			These are the tabs which will help you manage your Docker containers and
			images.
		</p>
		<ol>
			<figure>
				<img
					class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
					src={DockerDashboardTabs}
					alt="Docker Dashboard Tabs"
				/>
			</figure>
			<ul>
				<li>
					<code>Containers</code> tab will help you manage your Docker
					containers.
					<figure>
						<img
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
							src={DockerContainer}
							alt="Docker Container Management"
						/>
					</figure>
					<ol>
						<li>
							<code>Start | Restart Button</code> : This will start and stop the
							container in the background.
						</li>
						<li>
							<code>Delete Button</code> : This will delete the container and its
							data. It will not be recoverable.
						</li>
						<li>
							<code>More Action Button</code> : This will take you to this documentation
							page talking about the every action you could do.
						</li>
					</ol>
				</li>
				<li>
					<code>Images</code> tab will help you manage your Docker images.
					<figure>
						<img
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
							src={DockerImage}
							alt="Docker Images Management"
						/>
					</figure>
					<ol>
						<li>
							<code> Build Button </code>
							: This will let you build a container from an image but cannot customize
							the credential.
						</li>
						<li>
							<code> Delete Button </code>
							: This will delete the image and its data. It will not be recoverable.
						</li>
						<li>
							<code>More Action Button</code>
							: This will take you to this documentation page talking about the every
							action you could do.
						</li>
					</ol>
				</li>
				<li>
					<code>Pulling Images</code> tab will let you download Docker images
					from the registry.
					<figure>
						<img
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
							src={DockerPullingImage}
							alt="Docker Pulling Images"
						/>
					</figure>
					<ol>
						<li>
							<code>Search Input</code> : This will let you search for Docker Images
							directly from official Docker Registry.
						</li>
						<li>
							<code>Download Button</code> : This will let you download the image
							from the registry into your local. You can find the downloaded images
							in the images tab.
						</li>
					</ol>
				</li>
				<li>
					<code>System Terminal</code> tab will let you access the system
					terminal.
					<figure>
						<img
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
							src={DockerTerminal}
							alt="Docker Terminal"
						/>
					</figure>
					<ol>
						<li>
							<code>Command Input</code> : This will let you access the system terminal.
							You can also use the system built in terminal and the result will be
							the same. This wil let you have more control and customizability from
							the docker management.
						</li>
					</ol>
				</li>
			</ul>
		</ol>

		<li>
			Real World Example (Getting Started)
			<ol>
				<li>
					Postgres
					<div class="flex overflow-hidden">
						<figure
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
						>
							{@html PostgreSQLSvg('size-20')}
						</figure>
					</div>
					<ul>
						<li>
							Pull Image : <code>docker pull postgres</code>
						</li>
						<li>
							Build container : <code
								>docker run --name postgres -e POSTGRES_PASSWORD=postgres -p
								5432:5432 -d postgres</code
							>
						</li>
						<li>
							Connection URL: <code
								>postgres://postgres:postgres@localhost:5432</code
							>
						</li>
					</ul>
				</li>
				<li>
					MongoDB
					<div class="flex overflow-hidden">
						<figure
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
						>
							{@html MongoDBSvg('size-20')}
						</figure>
					</div>
					<ul>
						<li>
							Pull Image : <code>docker pull mongo</code>
						</li>
						<li>
							Build container : <code
								>docker run --name mongo -p 27017:27017 -e
								MONGO_INITDB_ROOT_USERNAME=mongo -e
								MONGO_INITDB_ROOT_PASSWORD=mongo -d mongo</code
							>
						</li>
						<li>
							Connection URL: <code>mongodb://mongo:mongo@localhost:27017</code>
						</li>
					</ul>
				</li>
				<li>
					MySQL
					<div class="flex overflow-hidden">
						<figure
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
						>
							{@html MySQLSvg('size-20')}
						</figure>
					</div>
					<ul>
						<li>
							Pull Image : <code>docker pull mysql</code>
						</li>
						<li>
							Build container : <code>
								docker run --name mysql -p 3306:3306 -e MYSQL_USER=mysql -e
								MYSQL_PASSWORD=mysql -e MYSQL_ROOT_PASSWORD=mysql -d mysql
							</code>
						</li>
						<li>
							Connection URL: <code>mysql://root:mysql@localhost:3306</code>
						</li>
					</ul>
				</li>
				<li>
					SQLite
					<div class="flex overflow-hidden">
						<figure
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
						>
							{@html SQLiteSvg('size-20')}
						</figure>
					</div>
					<ul>
						<li>
							Choose a local file that is SQLite which you want to use. (.db,
							.sqlite)
						</li>
					</ul>
				</li>
				<li>
					MariaDB
					<div class="flex overflow-hidden">
						<figure
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
						>
							{@html MariaDBSvg('size-20')}
						</figure>
					</div>
					<ul>
						<li>
							Pull Image : <code>docker pull mariadb</code>
						</li>
						<li>
							Build container : <code>
								docker run --name mariadb -p 3306:3306 -e MYSQL_USER=mariadb -e
								MYSQL_PASSWORD=mariadb -e MYSQL_ROOT_PASSWORD=mariadb -d mariadb
							</code>
						</li>
						<li>
							Connection URL: <code>mysql://root:mysql@localhost:3306</code>
						</li>
					</ul>
				</li>
				<li>
					MSSQL
					<div class="flex overflow-hidden">
						<figure
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
						>
							{@html MicrosoftSqlServerSvg('size-20')}
						</figure>
					</div>
					<ul>
						<li>
							Pull Image : <code
								>docker pull
								mcr.microsoft.com/mssql/server:2022-CU18-ubuntu-22.04</code
							>
						</li>
						<li>
							Build Container : <code>
								docker run -e "ACCEPT_EULA=Y" -e "MSSQL_SA_PASSWORD=Mssql123!"
								-p 1433:1433 --name mssql --hostname mssql -d
								mcr.microsoft.com/mssql/server:2022-CU18-ubuntu-22.04
							</code>
						</li>
						<li>
							Connection URL: <code>mssql://sa:Mssql123!@localhost:1433</code>
						</li>
					</ul>
				</li>
			</ol>
		</li>
	</ol>

	<h3 class="text-primary">## Workspace Dashboard</h3>

	<p>
		This page composed of three main components : <code>action menu</code>,
		<code>file tree</code>, <code>editor</code>.
	</p>
	<ol>
		<li>
			Action Menu
			<figure>
				<img
					class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
					src={WorkspaceActionMenu}
					alt="Workspace Action Menu"
				/>
			</figure>
			<ul>
				<li>
					<code>Add Button</code> : this is about adding a database connection
					to the application. When clicking the button. The dialog will open
					letting you choose the preferred datasource.
					<figure>
						<img
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
							src={WorkspaceSelectDatabase}
							alt="Workspace Datasource Selection"
						/>
					</figure>
					<ol>
						<li>
							<code>Cancel Button</code> : this will cancel the adding process, and
							closing the dialog
						</li>
						<li>
							<code>Connect Button</code> : this will confirm the datasource, and
							a dialog will pop up asking you to fill the url credential of the database.
							<figure>
								<img
									class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm "
									src={WorkspaceConnectDatabase}
									alt="Workspace Connect Database"
								/>
								</figure>	
								First you will need to fill the connection name and the database url. The connection name is the name you assign to this database url. Only the connection name will be easily be seen instead of database url. Make sure it is memorable and unique. The database url must follow the format of the given placeholder in order to be able to connect. 
								
								<ul>
									<li>
										<code>Cancel Button</code> : this will cancel the connecting process and return to the selecting datasource dialog.
									</li>
									<li>
										<code>Test Connection Button</code> : this will test the connection to the database url. If the connection is successful, a success message will be shown. If not, an error message will be shown. This will not save the connection to the application.
									</li>
									<li>
										<code>Connect Button</code> : this will connect to the database url and save the connection to the application. If the connection is successful, a success message will be shown. If not, an error message will be shown.
									</li>
								</ul>
							</li>
						
					</ol>
				</li>
				<li>
					<code>Refresh Button</code> : this will refresh the entire current page including database connections
				</li>
				<li>
					<code>Remove Button</code> : this will prompt you with the list of database connection you want to delete.
					the application.
					<figure>
						<img
							class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
							src={WorkspaceSelectRemoveDatabase}
							alt="Workspace Remove Database"
						/>
						</figure>	
						<ol>
							<li>
								<code>Remove Button</code> : this is for confirming the choice of removing database connection.
								<figure>
									<img src={WorkspaceRemoveDatabase} alt="Workspace Remove Database" class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"/>
									</figure>
									In order to remove the database, you will need to type in the database url of the connection to confirm the removal. This action is irreversible and will delete the connection from the application.
							</li>
						</ol>
				</li>
			</ul>
		</li>
		<li>File Tree
			<figure>
				<img
					class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
					src={WorkspaceFileTree}
					alt="Workspace File Tree"
				/>
			</figure>
			You can see the list of database connections you have added to the application.  You can click on the database connection to open it in the editor. You can also expand to see the schema preview of the database you have focus.
			<ul>
				<li>
					<code>Right Click</code> : you can take a look at the ER diagram of the database and share database connection. Check Feature if the datasource is supported by the application. Make sure you have to right click at the database level.
					<figure>
						<img src={WorkspaceERDiagram} alt="Workspace ER Diagram" class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm" />
					</figure>
					You can drag around the table and the relationship arrow will always follow scroll in and out to zoom in and out, you can move and play around. 
					<ul>
						<code>Reset View</code> : this will reset the view port back to the original position.
					</ul>
					<figure>
						<img src={WorkspaceShareDatabase} alt="Workspace Share Connection" class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm" />
					</figure>
					You can see your database connection and copy the link to share it with others. If the database is localhost, it will also show you with your system ip address.
				</li>
				
		</li>
		<li>Editor
			<figure>
				<img
					class="bg-base-200 rounded-box my-3 mr-3 px-5 py-3 shadow-sm"
					src={WorkspaceEditor}
					alt="Workspace Editor"
				/>
			</figure>

			You can see the which database is in focus before proceeding to write SQL queries. You can also see the list of tables in the database and the columns in the table. 

			<ul>
				<li>
					<code>New File Button</code> : First we will need to create a new file to write our queries. Give it a name and confirm the creation.
					
					
				</li>
				<li>

					<code>Edit File Button</code> : You can edit the file name if you want to change it.
				</li>
				<li>

					<code>Delete File Button</code> : You can delete the file if you no longer need it. This action is irreversible and will delete the file from the application.
				</li>

				<li>
					<code>Select File Combo Box</code> : You can see the list of file you have created and you can select the file to open it in the editor. You will see query blocks if you have written queries in the file.
				</li>
					<ul>
						<li>
							<code>New Query Block	Button </code> : You will have to create a query block in order to write your queries. You can create multiple query blocks in a file and will be saved in the application. 
						</li>
						Once you have created a query block, you can write your queries in the editor. Run Block button and Delete Block button will also appear.
						<ul>
							<li>
								<code>
									Run Block Button
								</code> : this will run the query block and show the result in the result area. 
								</li>	
								<li>
									<code>

										Delete Block Button 
									</code> : this will delete the query block and its data. It will not be recoverable.
								</li>
								<li>
									<code>
										Select Language Support Combo Box
									</code> : this will let you select the language support for the query block. The application will try to detect the language support automatically but you can also select it manually. It supports Markdown (MD), SQL and JSON.
								</li>
						</ul>
					</ul>
			</ul>
		</li>
	</ol>
	<h2 class="text-primary">
		## More Information

	</h2>
	<p class="">
		Check Out About Page to contact with the developers. <a href="/app/documentation-dashboard/about-documentation" class="link link-accent"> Go to About Page -> </a>
	</p>
</article>
