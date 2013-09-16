<?php

$iframe = $_GET['iframe'];

?><html>
<body>

<form action="/iframe.php" method="GET">
<input type="text" name="iframe" value="<?php echo $iframe; ?>"/>
<input type="submit">
</form>

<iframe style="width: 500px; height: 300px; border: 1px solid black;" src="<?php echo $iframe; ?>"></iframe>

</body>
</html>