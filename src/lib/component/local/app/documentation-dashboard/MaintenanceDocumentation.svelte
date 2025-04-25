<article class="animate-fade-in-up prose prose-xs lg:prose-sm !max-w-none">
	<h1 class="text-primary"># Maintenance</h1>
	<p>The application is built using a full stack with these technologies :</p>
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
		<li>
			<span class="font-semibold">Tauri</span> - Desktop Development
		</li>
		<li>
			<span class="font-semibold">Bollard</span> - Managing Docker Services
		</li>
		<li>
			<span class="font-semibold">SQLx</span> - Database Query Builder
		</li>
	</ul>
</article>

<main
	class=" animate-fade-in-up flex flex-col gap-16 px-8 text-sm sm:text-base"
>
	<section class="space-y-4">
		<h1 class="text-4xl font-extrabold text-blue-600">
			Database Helper Application
		</h1>
		<p class="">
			This application is built using a modern full-stack architecture
			leveraging a wide range of technologies:
		</p>
		<ul class="list-inside list-disc space-y-2">
			<li>
				<span class="font-semibold">SvelteKit</span> – Full-stack development
			</li>
			<li>
				<span class="font-semibold">TailwindCSS & DaisyUI</span> – Styling framework
			</li>
			<li>
				<span class="font-semibold">DrizzleORM</span> – App's internal database schema
				& migrations
			</li>
			<li>
				<span class="font-semibold">TypeORM</span> – User-managed database connections
			</li>
			<li>
				<span class="font-semibold">Tauri</span> – Compiles the app into a cross-platform
				executable
			</li>
			<li>
				<span class="font-semibold">Playwright</span> – End-to-end testing framework
			</li>
			<li><span class="font-semibold">ESLint</span> – Code linting</li>
			<li><span class="font-semibold">Prettier</span> – Code formatting</li>
		</ul>
	</section>

	<section class="space-y-4">
		<h2 class="text-2xl font-bold" id="prerequisites">Prerequisites</h2>
		<p>Before starting, make sure the following dependencies are installed:</p>
		<ul class="list-inside list-disc">
			<li>
				Node.js – <a href="https://nodejs.org" class="text-blue-600 underline"
					>Installation Guide</a
				>
			</li>
			<li>
				Rust – <a
					href="https://www.rust-lang.org/tools/install"
					class="text-blue-600 underline">Installation Guide</a
				>
			</li>
			<li>
				Tauri Setup – Follow the <a
					href="https://tauri.app/v1/guides/getting-started/prerequisites"
					class="text-blue-600 underline">official guide</a
				>
			</li>
			<li>Tauri CLI – <code>npm install -g @tauri-apps/cli</code></li>
		</ul>
	</section>

	<section class="space-y-4">
		<h2 class="text-2xl font-bold" id="project-setup">Project Setup</h2>
		<p>To initialize your project:</p>
		<ol class="list-inside list-decimal space-y-1">
			<li>
				Create the SvelteKit app: <code
					>npx create-svelte@latest database-helper-application</code
				>
			</li>
			<li>
				Select Prettier, ESLint, Vitest, Playwright, Drizzle, Adapter, and
				Tailwind during setup
			</li>
			<li>Install DaisyUI and configure it in <code>app.css</code></li>
			<li>Initialize Tauri: <code>npx tauri init</code></li>
			<li>
				Fill in config (name, build path, scripts) and update <code
					>package.json</code
				> with run scripts
			</li>
		</ol>
	</section>

	<section class="space-y-4">
		<h2 class="text-2xl font-bold" id="folder-structure">Folder Structure</h2>
		<p>The project structure is modular and scalable:</p>
		<ul class="list-inside list-disc space-y-1">
			<li><code>src/</code> contains all core files:</li>
			<ul class="list-inside list-disc pl-6">
				<li>
					<code>routes/</code> – Main pages (auth/app) using component-only structure
				</li>
				<li>
					<code>lib/components/</code> – Split into global (toolbar, dock), local
					(per route), and library (external tools)
				</li>
				<li><code>lib/assets/</code> – Images, fonts, styles, icons</li>
				<li>
					<code>lib/env/</code> – Environment variables using Svelte's built-in
					<code>$env</code>
				</li>
				<li><code>lib/models/</code> – Types, enums, static data arrays</li>
				<li>
					<code>lib/server/</code> – Contains <code>db/</code>, schema, and
					server logic (no client access)
				</li>
				<li>
					<code>lib/service/</code> – Contains sensitive functions: queries, fetchers,
					API handlers
				</li>
				<li>
					<code>lib/store/</code> – Separated into <code>safe</code>
					(cookies/localStorage),
					<code>state</code>
					(Svelte reactive vars), and <code>stalker</code> (background processes
					like Docker checks)
				</li>
				<li>
					<code>lib/tool/</code> & <code>lib/util/</code> – Project-specific vs.
					reusable utilities
				</li>
			</ul>
			<li>
				<code>src-tauri/</code> for Tauri:
				<ul class="list-inside list-disc pl-6">
					<li><code>models/</code> – Enums and Rust structs</li>
					<li><code>modules/</code> – Rust logic and functionality</li>
				</ul>
			</li>
		</ul>
	</section>
</main>
