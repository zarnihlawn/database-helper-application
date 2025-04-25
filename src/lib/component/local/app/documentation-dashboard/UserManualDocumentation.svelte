<script lang="ts">
	import DockerStartRestartClick from '$lib/asset/image/webp/docker-start-restart-click.webp';
	import DockerStopClick from '$lib/asset/image/webp/docker-stop-click.webp';
	import DockerDashboardTabs from '$lib/asset/image/webp/docker-dashboard-tabs.webp';
	import DockerContainer from '$lib/asset/image/webp/docker-container-tab.webp';
	import DockerImage from '$lib/asset/image/webp/docker-image-tab.webp';
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
						class="bg-base-200 rounded-box my-3 mr-3 flex flex-col justify-between gap-3 px-5 py-3 shadow-sm"
						src={DockerStartRestartClick}
						alt="Docker Start Restart Click"
					/>
				</figure>
			</li>
			<li>
				To Stop Docker, click the Stop button.
				<figure>
					<img
						class="bg-base-200 rounded-box my-3 mr-3 flex flex-col justify-between gap-3 px-5 py-3 shadow-sm"
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
					class="bg-base-200 rounded-box my-3 mr-3 flex flex-col justify-between gap-3 px-5 py-3 shadow-sm"
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
							class="bg-base-200 rounded-box my-3 mr-3 flex flex-col justify-between gap-3 px-5 py-3 shadow-sm"
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
							class="bg-base-200 rounded-box my-3 mr-3 flex flex-col justify-between gap-3 px-5 py-3 shadow-sm"
							src={DockerImage}
							alt="Docker Images Management"
						/>
					</figure>
				</li>
				<li>
					<code>Pulling Images</code> tab will let you download Docker images from
					the registry.
				</li>
				<li>
					<code>System Terminal</code> tab will let you access the system terminal.
				</li>
			</ul>
		</ol>

		<li>
			Real World Example (Getting Started)
			<ol>
				<li>Postgres</li>
				<li>MongoDB</li>
				<li>MySQL</li>
				<li>SQLite</li>
				<li>MariaDB</li>
				<li>MSSQL</li>
			</ol>
		</li>
	</ol>

	<h3 class="text-primary">## Workspace Dashboard</h3>
</article>
