export const ruLocalization = {
  translation: {
    layouts: {
      header: {
        t_language: "RU",
        t_name: "YDX",
        t_manager: "Менеджер",
        t_applications: "Приложения",
        t_documentation: "Документация",
      },
      footer: {
        t_copyright: "© 2026 YDX. Все права защищены.",
      },
    },
    components: {
      protected: {
        t_text: "Загрузка...",
      },
    },
    pages: {
      error: {
        t_title: "Страница не найдена",
        t_text: "Извините, страница, которую вы ищете, не существует.",
        t_btn: "На главную",
      },
      home: {
        t_text: "Скоро...",
      },
      auth: {
        t_title_signin: "YDX | Авторизация",
        t_title_signup: "YDX | Регистрация",
        t_subtitle_signin: "Редактируйте коллекции, учитесь на любом устройстве!",
        t_subtitle_signup: "Добавляйте коллекции и изучайте на любом устройстве!",
        benefits_signin: [
          "Нативный интерфейс на всех 8 платформах",
          "Работает молниеносно на любом устройстве",
          "Синхронизация между всеми устройствами",
          "Русский, English, 中文 — на всех платформах",
          "Простой и понятный интерфейс",
          "Нейросеть готовит контент за вас",
        ],
        benefits_signup: [
          "Создавайте карточки на любые темы",
          "Нативный интерфейс на всех 8 платформах",
          "Всё синхронизируется между устройствами",
          "Русский, English, 中文 — на всех платформах",
          "Ничего лишнего — только карточки",
          "Нейросеть поможет с наполнением",
          "Первые карточки — сразу после регистрации",
          "Никаких готовых курсов — только ваши интересы",
        ],
      },
      signin: {
        t_label_email: "Email",
        t_label_password: "Пароль",
        t_btn_submit: "Войти",
        t_success: "Вход выполнен успешно",
      },
      signup: {
        t_label_email: "Email",
        t_label_password: "Пароль",
        t_label_password_confirm: "Подтвердите пароль",
        t_btn_submit: "Зарегистрироваться",
        t_success: "Регистрация прошла успешно",
      },
    },
    // Localization form validate server/yup
    "Invalid email or password.": "Неверный email или пароль.",
    "Size must be between 3 and 250.": "Размер должен быть от 3 до 250.",
    "Must not be not blank.": "Не должно быть пустым.",
    "Must be a well-formed email address.":
      "Должен быть корректный email адрес.",
    "Size must be between 8 and 16.": "Размер должен быть от 8 до 16.",
  },
};
