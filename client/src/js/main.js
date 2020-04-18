(function(scope) {


  function mount(tree) {
    if (!tree.children || !tree.children.length) {
      return;
    }
    tree.children.forEach(item=>{
      tree.el.appendChild(item.el);
    });
    return mount(tree.children);
  }





  function render() {
    const title = document.createElement('h1');
    title.appendChild(document.createTextNode('Jarvis Inbound'));
    title.classList.add('text-center');
    const tagline = document.createElement('p');
    tagline.classList.add('text-center');
    tagline.appendChild(document.createTextNode('Hello World,using rust web view and vanilla js'));

    return {
      el: document.querySelector('main'),
      children: [
        {
          el: title
        },
        {
          el: tagline
        }
      ]
    };
  }



  const tree = render();

  mount(tree);
})(window)
