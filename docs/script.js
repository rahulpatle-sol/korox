(function () {
  "use strict";

  var menuToggle = document.querySelector(".menu-toggle");
  var navMenu = document.querySelector(".nav-menu");

  if (menuToggle && navMenu) {
    menuToggle.addEventListener("click", function () {
      var isOpen = navMenu.classList.toggle("is-open");
      menuToggle.setAttribute("aria-expanded", String(isOpen));
      menuToggle.querySelector("i").className = isOpen ? "ri-close-line" : "ri-menu-line";
    });

    navMenu.querySelectorAll("a").forEach(function (link) {
      link.addEventListener("click", function () {
        navMenu.classList.remove("is-open");
        menuToggle.setAttribute("aria-expanded", "false");
        menuToggle.querySelector("i").className = "ri-menu-line";
      });
    });
  }

  function copyText(text, button) {
    navigator.clipboard.writeText(text).then(function () {
      var label = button.querySelector("span");
      if (!label) return;
      var original = label.textContent;
      label.textContent = "Copied";
      setTimeout(function () { label.textContent = original; }, 1400);
    }).catch(function () {
      button.setAttribute("aria-label", "Copy failed");
    });
  }

  document.querySelectorAll(".copy-button").forEach(function (button) {
    button.addEventListener("click", function () {
      var block = button.closest(".code-block");
      var target = button.getAttribute("data-copy-target");
      var element = target ? document.getElementById(target) : block;
      var text = element ? (element.textContent || "").trim() : "";
      if (text) copyText(text, button);
    });
  });

  var sections = document.querySelectorAll("main section[id]");
  var navLinks = document.querySelectorAll(".nav-menu a[href^='#']");
  if ("IntersectionObserver" in window) {
    var sectionObserver = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        if (!entry.isIntersecting) return;
        navLinks.forEach(function (link) {
          link.classList.toggle("is-active", link.getAttribute("href") === "#" + entry.target.id);
        });
      });
    }, { rootMargin: "-28% 0px -62% 0px", threshold: 0 });
    sections.forEach(function (section) { sectionObserver.observe(section); });
  }

  document.addEventListener("pointermove", function (event) {
    document.body.classList.add("cursor-ready");
    document.body.style.setProperty("--cursor-x", event.clientX + "px");
    document.body.style.setProperty("--cursor-y", event.clientY + "px");
  }, { passive: true });

  if (window.gsap && window.ScrollTrigger) {
    gsap.registerPlugin(ScrollTrigger);
    var motionQuery = window.matchMedia("(prefers-reduced-motion: reduce)");
    if (!motionQuery.matches) {
      gsap.from("[data-gsap='hero-copy']", { opacity: 0, y: 34, duration: 1, ease: "power3.out" });
      gsap.from("[data-gsap='terminal']", { opacity: 0, x: 34, rotateY: 5, duration: 1.1, delay: .16, ease: "power3.out" });
      gsap.to(".hero-copy", { yPercent: -8, ease: "none", scrollTrigger: { trigger: ".hero", start: "top top", end: "bottom top", scrub: true } });
      gsap.to(".terminal-window", { yPercent: 11, ease: "none", scrollTrigger: { trigger: ".hero", start: "top top", end: "bottom top", scrub: true } });
    }
    gsap.utils.toArray("[data-gsap='reveal']").forEach(function (element) {
      gsap.from(element, { opacity: 0, y: 28, duration: .8, ease: "power3.out", scrollTrigger: { trigger: element, start: "top 84%" } });
    });
    gsap.utils.toArray("[data-gsap='card']").forEach(function (element, index) {
      gsap.from(element, { opacity: 0, y: 30, duration: .7, delay: (index % 3) * .08, ease: "power3.out", scrollTrigger: { trigger: element, start: "top 88%" } });
    });
  }
})();