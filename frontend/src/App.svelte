<script lang="ts">
  import {Router, Link, Route} from "svelte-routing";
  import LoginPage from "./lib/pages/LoginPage.svelte";
  import TimetableViewPage from "./lib/pages/TimetableViewPage.svelte";
  import RegisterPage from "./lib/pages/RegisterPage.svelte";
  import AboutPage from "./lib/pages/AboutPage.svelte";


</script>
<Router>
  <Route path="/login" component={LoginPage}/>
  <Route path="/timetable" component={TimetableViewPage}/>
  <Route path="/register" component={RegisterPage}/>
  <Route path="/about" component={AboutPage}/>
</Router>

<main>
</main>

<style>
</style>
