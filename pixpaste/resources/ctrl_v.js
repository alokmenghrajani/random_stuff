if (navigator.userAgent.indexOf('Firefox') != -1) {
  var e = document.createElement('div');
  document.body.appendChild(e);
  e.id = 'editor';
  e.contentEditable = true;
  e.style.position = 'absolute';
  e.style.left = '-10000px';
  e.style.top = '0px';

  setInterval('document.getElementById("editor").focus()', 1);

  document.body.addEventListener("paste", function(e) {
    setTimeout(function() {
      var e = document.getElementById('editor');
      for (var i=0; i<e.children.length; i++) {
        var node = e.children[i];
        if (node.nodeName == 'img') {
          document.getElementById('img_data').value = node.src;
          document.getElementById('preview').src = node.src;
          document.getElementById('btn').className = "btn primary";
          document.getElementById('error').style.display = 'none';
          e.innerHTML = '';
          return;
        }
      }
      e.innerHTML = '';
      document.getElementById('error').innerHTML = "Sorry, paste failed. Did not get image data!";
      document.getElementById('error').style.display = '';
      document.getElementById('btn').className = "btn";
      document.getElementById('img_data').value = '';
      document.getElementById('preview').src = 'resources/preview.png';
    }, 1);
  });
} else {
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
    document.getElementById('img_data').value = '';
    document.getElementById('preview').src = 'resources/preview.png';
  });
}
