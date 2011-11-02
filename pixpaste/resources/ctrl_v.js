document.body.addEventListener("paste", function(e) {
  for (var t in e.clipboardData.types) {
    type = e.clipboardData.types[t];
    if (type == 'image/png') {
      file = e.clipboardData.items[t].getAsFile();
      reader = new FileReader();
      reader.onload = function(e) {
        document.getElementById('img_data').value = e.target.result;
        document.getElementById('preview').src = e.target.result;
      };
      reader.readAsDataURL(file);
      document.getElementById('btn').className = "btn primary";
      document.getElementById('error').style.display = 'none';
      return;
    }
  }
  document.getElementById('error').innerHTML = "Sorry, paste failed. Got "+
  e.clipboardData.types.join(", ") + ". Was expecting image/png data!"
  document.getElementById('error').style.display = '';
  document.getElementById('btn').className = "btn";
});

