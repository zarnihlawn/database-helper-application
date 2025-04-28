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
							<code>Connect Button</code> : this will confirm the datasource, and
							a dialog will pop up asking you to fill the url credential of the database.
						</li>
						<li>
							<code>Cancel Button</code> : this will cancel the adding process, and
							closing the dialog
						</li>
					</ol>
				</li>
				<li>
					<code>Remove Button</code> : this will remove the database connection from
					the application.
				</li>
			</ul>
		</li>
		<li>File Tree</li>
		<li>Editor</li>
	</ol>
</article>
