<article class="animate-fade-in-up prose prose-xs lg:prose-sm !max-w-none">
	<h1 class="text-primary"># Maintenance</h1>
	<h2 class="text-primary">## Prerequisites</h2>
	<ul>
		<li>
			<span class="font-semibold">NodeJS</span> - v20.12.0
		</li>
		<li>
			<span class="font-semibold">Rust</span> - v1.85.0
		</li>
		<li>
			<span class="font-semibold">Tauri</span> -
			<a
				href="https://v2.tauri.app/start/prerequisites/"
				class="link link-secondary">https://v2.tauri.app/start/prerequisites/</a
			>
		</li>
	</ul>

	<h2 class="text-primary">## Getting Started</h2>
	<p>
		To get started with the Database Helper Application, follow these steps :
	</p>
	<ol>
		<li>Unzip the application</li>
		<li>Open a terminal with the application folder</li>
		<li>Install dependencies and Run the Application</li>
		<ul>
			<li>
				<span class="font-semibold">npm i </span>
			</li>
			<li>
				<span class="font-semibold">npm run tauri dev </span>
			</li>
		</ul>
	</ol>

	<h2 class="text-primary">## Technologies</h2>
	<p>The application is built using a full stack with these technologies :</p>
	<h3 class="text-primary">### Front End</h3>
	<ul>
		<li>
			<span class="font-semibold">SvelteKit with Static Adapter</span> - Front End
			Development
		</li>
		<li>
			<span class="font-semibold">Typescript</span> - Type Checking
		</li>
		<li>
			<span class="font-semibold">TailwindCSS and DaisyUI</span> - Styling Components
		</li>
		<li>
			<span class="font-semibold">DrizzleORM</span> - Database Schema and Migrations
		</li>
		<li>
			<span class="font-semibold">Playwright</span> - End-to-End Testing
		</li>
		<li>
			<span class="font-semibold">Vitest</span> - Unit Testing
		</li>
		<li>
			<span class="font-semibold">Prettier</span> - Code Formatting
		</li>
		<li>
			<span class="font-semibold">ESLint</span> - Code Linting
		</li>
	</ul>

	<h3 class="text-primary">### Back End</h3>
	<ul>
		<li>
			<span class="font-semibold">Tauri</span> - Desktop Development
		</li>
		<li>
			<span class="font-semibold">Bollard</span> - Managing Docker Services
		</li>
		<li>
			<span class="font-semibold">SQLx</span> - (Postgres, MySql, SQLite) Query Builder
		</li>
		<li>
			<span class="font-semibold">Tiberius</span> - MSSQL Query Builder
		</li>
		<li>
			<span class="font-semibold">Mongodb</span> - MongoDB Query Builder
		</li>
		<li>
			<span class="font-semibold">Plugin</span> - Tauri Plugins Controlling Application
		</li>
	</ul>

	<h3 class="text-primary">### Database</h3>
	<ul>
		<li>
			<span class="font-semibold">SQLite</span> - Database will be created in the user system. For Windows: "C:\Users\Asus\.config\database-helper-application\local.db"
		</li>
	</ul>


	<h2 class="text-primary">## Folder Structure</h2>
	<ul>
		<li>
			<span class="font-semibold">.github</span>
			<ul>
				<li>
					<span class="font-semibold">workflows</span>
					<ul>
						<li>
							<span class="font-semibold">build.yml</span> - instruction build script
							for cross platform taking advantages of Github Actions.
						</li>
					</ul>
				</li>
			</ul>
		</li>
		<li>
			<span class="font-semibold">.svelte-kit</span> - auto generated folder when
			running application
		</li>
		<li>
			<span class="font-semibold">.vscode</span> - workspace settings such as themes
			and extensions
		</li>
		<li>
			<span class="font-semibold">build</span> - auto generated folder after building
			application
		</li>
		<li>
			<span class="font-semibold">e2e</span> - testing folder using playwright
		</li>
		<li>
			<span class="font-semibold">node_modules</span> - node modules folder
		</li>
		<li>
			<span class="font-semibold">src</span> - frontend sveltekit code
			<ul>
				<li>
					<span class="font-semibold">lib</span> - stands for library, containing
					reusable components
				</li>
				<li>
					<span class="font-semibold">routes</span> - endpoint routes or pages url
				</li>
			</ul>
		</li>
		<li>
			<span class="font-semibold">src-tauri</span> - backend tauri rust code
			<ul>
				<li>
					<span class="font-semibold">capabilities</span> - permission what this
					app can do
				</li>
				<li>
					<span class="font-semibold">gen</span>
				</li>
				<li>
					<span class="font-semibold">icons</span> - icons pack with different sizes
					for different platforms
				</li>
				<li>
					<span class="font-semibold">src</span> - rust source code
				</li>
				<li>
					<span class="font-semibold">target</span> - compiled rust (tauri) code
				</li>
			</ul>
		</li>
		<li>
			<span class="font-semibold">static</span> - static assets (also known as public
			folder)
		</li>
	</ul>
</article>
