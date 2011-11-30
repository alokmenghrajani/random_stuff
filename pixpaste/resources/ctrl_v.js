(function(){
  var file_to_img = function(file) {
    reader = new FileReader();
    reader.onload = function(e) {
      document.getElementById('preview').src = e.target.result;
    };
    // todo: handle case where file isn't an image...
    reader.readAsDataURL(file);
    document.getElementById('btn').className = "btn primary";
    document.getElementById('error').style.display = 'none';
  }
  var noop_handler = function(e) {
    e.stopPropagation();
    e.preventDefault();
  };
  document.addEventListener("dragenter", noop_handler, false);
  document.addEventListener("dragexit", noop_handler, false);
  document.addEventListener("dragover", noop_handler, false);
  document.addEventListener("drop", function(e) {
    e.preventDefault();
    var file = e.dataTransfer.files[0];
    file_to_img(file);
  }, false);

  dialogOpen = false;
  var prompt_file_upload = function(e) {
    if (!dialogOpen) {
      dialogOpen = true;
      document.getElementById('file').click();
      console.log(document.getElementById('file').files);
      file_to_img(document.getElementById('file').files[0]);
      dialogOpen = false;
    }
  }
  document.addEventListener("click", prompt_file_upload, false);

  if (navigator.userAgent.indexOf('Firefox') != -1) {
    var e = document.createElement('div');
    document.body.appendChild(e);
    e.id = 'editor';
    e.contentEditable = true;
    e.style.position = 'fixed';
    e.style.left = '-10000px';
    e.style.top = '0px';

    setInterval('document.getElementById("editor").focus()', 1);

    document.body.addEventListener("paste", function(e) {
      setTimeout(function() {
        var e = document.getElementById('editor');
        for (var i=0; i<e.children.length; i++) {
          var node = e.children[i];
          if (node.nodeName == 'img') {
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
        document.getElementById('preview').src = 'resources/preview.png';
      }, 1);
    });
  } else {
    document.body.addEventListener("paste", function(e) {
      for (var t in e.clipboardData.types) {
        var type = e.clipboardData.types[t];
        if (type == 'image/png') {
          file = e.clipboardData.items[t].getAsFile();
          file_to_img(file);
          return;
        }
      }
      document.getElementById('error').innerHTML = "Sorry, paste failed. Got "+
      e.clipboardData.types.join(", ") + ". Was expecting image/png data!"
      document.getElementById('error').style.display = '';
      document.getElementById('btn').className = "btn";
      document.getElementById('preview').src = 'resources/preview.png';
    });
  }
})();

