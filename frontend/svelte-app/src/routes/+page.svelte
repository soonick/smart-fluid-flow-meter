<script lang="ts">
	function login(e: Event) {
		const form = document.getElementById('login-form') as HTMLFormElement;

		if (form.checkValidity()) {
			alert('is valid');
		} else {
			form.reportValidity();
		}

		e.preventDefault();
	}
</script>

<div class="login">
	<h1>Login</h1>
	<form action="#" method="POST" id="login-form">
		<div class="form-group">
			<label for="email">Email</label>
			<input type="email" id="email" name="email" required />
		</div>
		<div class="form-group">
			<label for="password">Password</label>
			<input type="password" id="password" name="password" required />
		</div>
		<div class="form-group">
			<button type="submit" onclick={(e: Event) => login(e)}>Log In</button>
		</div>
	</form>
</div>

<style>
	.login {
		border: 1px solid var(--primary-color);
		border-radius: 5px;
		margin: 0 auto;
		margin-top: 5rem;
		margin-bottom: 5rem;
		padding: 1rem;
		width: 24rem;
	}

	h1 {
		font-size: 1.3em;
		text-align: center;
		padding-bottom: 1.5rem;
	}

	label,
	input {
		display: block;
	}

	input {
		border: 1px solid var(--primary-color);
		width: 100%;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	button {
		background: var(--primary-color);
		color: var(--secondary-color-weak);
		border-radius: 5px;
		display: block;
		margin: 0 auto;
		padding: 0.5rem;
	}
</style>
