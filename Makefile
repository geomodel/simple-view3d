CHECK_GIT_STATUS = git status -s | sed 's/"/|/g'


help:
	@cat Makefile

# # # # # # # #
pull:
	@git pull

savetogit: git.pushall
git.pushall: git.commitall
	@git push
git.commitall: git.addall
	@echo '--> COMMIT if STATUS allows..'
	@if [ -n "$(shell $(CHECK_GIT_STATUS))" ]; \
		then \
			git commit -m 'saving'; \
		else \
			echo '--- nothing to commit'; \
	fi
git.addall:
	@git add .

